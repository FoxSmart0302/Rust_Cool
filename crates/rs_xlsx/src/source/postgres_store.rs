use crate::error::XResult;
use crate::product::{FlatProduct, Product};
use crate::results::{table_name, TableType};
use crate::source::Source;
use sqlx::{Pool, Postgres};

pub struct PostgresStore {
    pub db: Pool<Postgres>,
    pub products_table: String,
    pub errors_table: String,
}

impl PostgresStore {
    pub fn new(pool: Pool<Postgres>, scan_id: i64) -> Self {
        Self {
            db: pool,
            products_table: table_name(scan_id, TableType::Products),
            errors_table: table_name(scan_id, TableType::Errors),
        }
    }
}

#[async_trait::async_trait]
impl Source for PostgresStore {
    async fn read(&self) -> XResult<Vec<Product>> {
        let mut products: Vec<FlatProduct> =
            sqlx::query_as(&format!("select * from \"{}\"", self.products_table))
                .fetch_all(&self.db)
                .await?;
        sqlx::query_as::<_, FlatProduct>(&format!("select * from \"{}\"", self.errors_table))
            .fetch_all(&self.db)
            .await?
            .into_iter()
            .for_each(|x| products.push(x));

        Ok(products.into_iter().map(|x| x.into()).collect())
    }
}
