mod insert;
mod populate_columns;

use crate::error::XResult;
use crate::populate::insert::{insert, InsertParams};
use crate::populate::populate_columns::get_columns_list_as_string;
use crate::product::Product;
use crate::results::{table_name, TableType};
use crate::source::Source;
use crate::usage_tracker::UsageTracker;
use itertools::Itertools;
use rs_models::Scan;
use sqlx::{Pool, Postgres};
#[cfg(test)]
use uuid::Uuid;

const INSERT_BATCH_SIZE: usize = 100;

/// Try to populate the local PG table with products from the given [Source].
///
/// If the given table already exists, then this is a no-op.
pub async fn populate(
    pg: Pool<Postgres>,
    scan: &Scan,
    source: impl Source,
    tracker: &impl UsageTracker,
) -> XResult<()> {
    let products_table = table_name(scan.id, TableType::Products);

    let r = _populate(pg, scan, source, tracker).await;
    tracker.release(&products_table).await?;
    tracker.touch(&products_table, scan.account_id).await?;
    r
}

async fn _populate(
    pg: Pool<Postgres>,
    scan: &Scan,
    source: impl Source,
    tracker: &impl UsageTracker,
) -> XResult<()> {
    let products_table = table_name(scan.id, TableType::Products);
    let errors_table = table_name(scan.id, TableType::Errors);

    // By waiting for a lock, we are making sure that we don't have another
    // populate process running in parallel.
    tracker.wait_for_lock(&products_table).await?;

    let products_table_exists = table_exists(&pg, &products_table).await?;
    let errors_table_exists = table_exists(&pg, &errors_table).await?;

    // Nothing to do, both tables already exist
    if products_table_exists && errors_table_exists {
        return Ok(());
    }

    // Load products
    let products = source.read().await?;
    println!("Loaded {} products", products.len());

    if !products_table_exists {
        populate_table(&pg, scan, TableType::Products, &products).await?;
    }

    if !errors_table_exists {
        populate_table(&pg, scan, TableType::Errors, &products).await?;
    }

    Ok(())
}

async fn table_exists(pg: &Pool<Postgres>, table_name: &str) -> XResult<bool> {
    let r = sqlx::query_scalar!(
        "select exists(select * from pg_tables where schemaname = 'public' and tablename = $1)",
        table_name
    )
    .fetch_one(pg)
    .await?;

    match r {
        Some(r) => Ok(r),
        None => Ok(false),
    }
}

/// Creates the table and populates it from the passed products vec.
async fn populate_table(
    pg: &Pool<Postgres>,
    scan: &Scan,
    table_type: TableType,
    products: &[Product],
) -> XResult<()> {
    let table_name = table_name(scan.id, table_type);

    create_table(pg, &table_name).await?;

    let chunks = products
        .iter()
        .filter(|x| table_type.allowed(x))
        .chunks(INSERT_BATCH_SIZE);

    let params = InsertParams::new(pg.clone(), &table_name);

    let mut count = 0;
    for chunk in chunks.into_iter() {
        // insert_via_copy(&params, chunk, &mut count).await?;
        insert(&params, chunk, &mut count).await?;
    }

    println!("Inserted {} rows for {:?}", count, table_type);

    Ok(())
}

async fn create_table(pg: &Pool<Postgres>, table_name: &str) -> XResult<()> {
    let cols = get_columns_list_as_string();
    let query = format!("create unlogged table \"{table_name}\" ({cols})");

    sqlx::query(&query).execute(pg).await?;

    Ok(())
}

#[cfg(test)]
mod populate_tests {
    use crate::error::XResult;
    use crate::populate::insert::{insert, InsertParams};
    use crate::populate::{create_table, drop_table, populate};
    use crate::product::{FlatProduct, Product};
    use crate::source::RemoteGzippedJsonStore;
    use crate::usage_tracker::RedisUsageTracker;
    use dotenvy::dotenv;
    use rs_models::{get_pool, Repo};
    use sqlx::{Pool, Postgres};
    use uuid::Uuid;

    #[tokio::test]
    #[ignore]
    async fn it_works() -> XResult<()> {
        dotenv()?;

        let db = get_pool("rocketsource").await?;
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let c = client.get_tokio_connection_manager().await?;
        let tracker = RedisUsageTracker::new(c);
        let repo = Repo::new(db.clone());

        let scan = repo.scans.find(70).await?;

        sqlx::query("drop table if exists scan_70")
            .execute(&db)
            .await?;
        sqlx::query("drop table if exists scan_70_errors")
            .execute(&db)
            .await?;

        let source = RemoteGzippedJsonStore::new(scan.results.clone());

        populate(db, &scan, source, &tracker).await?;

        Ok(())
    }

    /// This is testing that we can convert to and from the FlatProduct
    /// struct. [FlatProduct] is an intermediary struct used to model
    /// [Product] in the database.
    #[test]
    fn it_can_convert() {
        for _ in 0..1000 {
            let p = Product::fake_no_extra();
            let converted = FlatProduct::from(p.clone());
            let converted_back = Product::from(converted.clone());
            assert_eq!(p, converted_back);
        }
    }

    /// Generates a bunch of fake [Product], inserts them into the database,
    /// pulls them back out, and makes sure that they are equal.
    #[tokio::test]
    async fn gen_fake() -> XResult<()> {
        dotenv().unwrap();

        // The `Env` lets us tweak what the environment
        // variables to read are and what the default
        // value is if they're missing
        // let env = Env::default()
        //     .filter_or("MY_LOG_LEVEL", "trace")
        //     .write_style_or("MY_LOG_STYLE", "always");
        // env_logger::init_from_env(env);

        let pool = get_pool("rocketsource").await?;
        let table_name = format!("test{}", Uuid::new_v4().to_string());

        // This wraps an inner function in order to make sure
        // that the table is dropped after the test is done.
        //
        // A normal Drop impl won't work here, as we need async
        // calls to drop the table and async is not supported
        // in drop.
        let r = _gen_fake(&pool, &table_name).await;
        drop_table(&pool, &table_name).await?;
        r
    }

    async fn _gen_fake(pool: &Pool<Postgres>, table_name: &str) -> XResult<()> {
        create_table(&pool, &table_name).await?;

        let mut products = Vec::from_iter((0..1000).map(|_| Product::fake_no_extra()));
        products.sort_by(|a, b| a.id.cmp(&b.id));

        // Insert into the database
        let params = InsertParams::new(pool.clone(), &table_name);

        let mut count = 0;
        // println!("Starting insert");
        insert(&params, products.iter(), &mut count).await?;
        // println!("Done insert");

        // Pull them back out
        let r = sqlx::query_as::<_, FlatProduct>(&format!(
            "select * from \"{}\" order by id asc",
            table_name
        ))
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Product>>();

        // Make sure they are equal
        assert_eq!(products, r);

        Ok(())
    }
}

#[cfg(test)]
async fn drop_table(pg: &Pool<Postgres>, table_name: &str) -> XResult<()> {
    let r = Uuid::parse_str(table_name);
    let r2 = Uuid::parse_str(&table_name[4..]);
    if r.is_err() && r2.is_err() {
        return Ok(());
    }

    // Drop table, since it's a valid UUID
    sqlx::query(&format!("drop table if exists \"{}\"", table_name))
        .execute(pg)
        .await?;

    Ok(())
}
