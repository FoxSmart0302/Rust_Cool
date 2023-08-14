use rs_models::{get_pool, Scan};

#[tokio::test]
#[ignore]
async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().unwrap();

    let pool = get_pool("rocketsource").await?;

    // Breaking ids = 7401, 6462, 57

    // let mut start = i64::MAX;
    let mut start = 57;
    let chunk = 10;

    for _ in 0..10 {
        println!("Starting on {}", start);
        let r: Vec<Scan> =
            sqlx::query_as("SELECT * from scans where id <= $1 order by id desc limit $2")
                .bind(start)
                .bind(chunk)
                .fetch_all(&pool)
                .await
                .unwrap();

        if r.is_empty() {
            break;
        }
        start = r.iter().last().unwrap().id - 1;
    }

    println!("All done!");

    Ok(())
}
