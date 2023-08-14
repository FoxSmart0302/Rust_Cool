use dotenvy::dotenv;
use rs_models::env;
use rs_xlsx::error::XResult;
use rs_xlsx::run;

#[tokio::main]
async fn main() -> XResult<()> {
    dotenv()?;

    let is_in_prod = env("APP_ENV") == "production";
    let file_appender = if is_in_prod {
        tracing_appender::rolling::never("", "/rs_xlsx.log")
    } else {
        tracing_appender::rolling::never("", "rs_xlsx.log")
    };

    tracing_subscriber::fmt()
        .json()
        .with_writer(file_appender)
        .init();

    run().await
}

#[cfg(test)]
mod dummy_run {
    use rs_models::{MultipackPrepCost, Scan, ScanOptions};
    use rs_xlsx::error::XResult;
    use rs_xlsx::result_request::RawResultRequest;
    use rs_xlsx::{generate, parse_file};
    use sqlx::types::Json;
    use std::io::Read;

    #[test]
    #[ignore]
    fn dummy_run() -> XResult<()> {
        let scan = Scan {
            id: 0,
            account_id: 0,
            marketplace_id: 0,
            status: 0,
            source_type_id: 0,
            source_id: 0,
            name: "".to_string(),
            products: 0,
            errors: 0,
            speed: 0,
            supplier_file: "".to_string(),
            results: "".to_string(),
            options: Some(Json(ScanOptions {
                name: "".to_string(),
                header: None,
                mapping: None,
                prep_cost: 1.0,
                amazon_vat: None,
                supplier_vat: None,
                custom_columns: vec![],
                marketplace_id: 0,
                discount_supplier: 123,
                multipack_override: None,
                input_shipping_rate: 12,
                multipack_prep_cost: MultipackPrepCost {
                    first_n: 3,
                    enabled: true,
                    cost_for_rest: 0.5,
                    cost_for_first_n: 0.75,
                },
                multipack_override_quantity: None,
            })),
            created_at: None,
            updated_at: None,
            filename: "".to_string(),
            deleted_at: None,
            user_id: 0,
            lines: 0,
        };

        let mut f = std::fs::File::open("fixtures/request_with_custom_col2.json")?;
        // let mut f = std::fs::File::open("fixtures/request_with_custom_col.json")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let r = serde_json::from_str::<RawResultRequest>(&s)?;
        let r = r.parse(&scan)?;

        let path = "fixtures/29581.json.gz";
        let items = parse_file(path)?;
        generate(scan, path, r.clone(), items).unwrap();

        Ok(())
    }

    // 21128

    // #[tokio::test]
    // #[ignore]
    // async fn prod_run() -> XResult<()> {
    //     run();
    //
    //     dotenv()?;
    //     let pool = get_pool("rocketsource").await?;
    //     let repo = Repo::new(pool);
    //
    //     let scan = repo.scans.find(21128).await?;
    //     println!("{:?}", scan);
    //
    //     let mut f = std::fs::File::open("fixtures/request.json")?;
    //     let mut s = String::new();
    //     f.read_to_string(&mut s)?;
    //     let r = serde_json::from_str::<ResultRequest>(&s)?;
    //
    //     let path = "fixtures/29581.json.gz";
    //     let items = parse_file(path)?;
    //     generate(&scan, path, r.clone(), items).unwrap();
    //
    //     Ok(())
    // }
}
