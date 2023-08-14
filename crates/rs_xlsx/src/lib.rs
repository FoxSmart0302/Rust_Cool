mod app;
pub mod error;
mod handlers;
mod inbound_eligibility;
mod middleware;
mod new_key_cache;
mod populate;
mod product;
pub mod result_request;
mod results;
mod source;
mod usage_tracker;
mod xlsx;

use crate::app::State;
use crate::error::{XError, XResult};
use crate::handlers::{enrich_pricing, export_xlsx, not_found};
use crate::product::Product;
use crate::result_request::ResultRequest;
use crate::xlsx::xlsx_builder::XlsxBuilder;
use actix_request_identifier::RequestIdentifier;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use flate2::read::GzDecoder;
use rs_models::{env, get_pool, Scan};
use sp_api::marketplaces::Marketplace;
use std::io::prelude::*;
use tokio::signal::ctrl_c;
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

pub async fn run() -> XResult<()> {
    dotenv()?;

    // let env_logger = Env::default()
    //     .filter_or("MY_LOG_LEVEL", "trace")
    //     .write_style_or("MY_LOG_STYLE", "always");
    // env_logger::init_from_env(env_logger);

    let pg_pool = get_pool(&env("DB_NAME")).await?;

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let redis = client.get_tokio_connection_manager().await?;

    let state = State::new(pg_pool, redis).await?;

    println!("Starting server");
    let server = HttpServer::new(move || {
        // Limit request payload size. This overrides the default of 2mb
        let json_cfg = web::JsonConfig::default().limit(128 * 1_048_576); // 128MB
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(json_cfg)
            // .service(hndl_promotion)
            // .service(hndl_start_update)
            .service(enrich_pricing)
            .service(export_xlsx)
            .wrap(RequestIdentifier::with_uuid())
            .default_service(web::route().to(not_found))
    });

    let port: u16 = std::env::var("RS_XLSX_PORT")
        .unwrap_or("85".to_owned())
        .parse()
        .unwrap();

    println!("Listening on port {}", port);

    let server = server
        .bind(("0.0.0.0", port))?
        .shutdown_timeout(90)
        .disable_signals()
        .run();

    let server_handle = server.handle();

    let server_task = tokio::spawn(server);

    let shutdown = tokio::spawn(async move {
        // listen for ctrl-c
        let mut s1 = signal(SignalKind::hangup()).unwrap();
        let mut s2 = signal(SignalKind::terminate()).unwrap();
        let mut s3 = signal(SignalKind::interrupt()).unwrap();
        let s4 = ctrl_c();

        tokio::select! {
            _ = s1.recv() => {
                println!("rcvd SIGHUP, shutting down")
            },
            _ = s2.recv() => {
                println!("rcvd SIGTERM, shutting down")
            },
            _ = s3.recv() => {
                println!("rcvd SIGINT, shutting down")
            },
            _ = s4 => {
                println!("rcvd ctrl+c, shutting down")
            },
        }
    });

    let _ = tokio::try_join!(shutdown).expect("unable to join tasks");

    // await shutdown of server
    let server_stop = server_handle.stop(true);
    server_stop.await;
    server_task.await??;

    println!("Cleanly exiting");

    Ok(())
}

pub fn parse_file(path: &str) -> XResult<Vec<Product>> {
    // Read a gzipped file
    let f = std::fs::File::open(path)?;
    let mut d = GzDecoder::new(f);
    let mut s = String::new();
    d.read_to_string(&mut s)?;

    // Split it into lines
    let mut out = vec![];
    for line in s.split('\n') {
        if line.trim().is_empty() {
            continue;
        }

        // And parse each line
        out.push(serde_json::from_str::<Product>(line)?);
    }

    Ok(out)
}

pub fn generate(scan: Scan, path: &str, r: ResultRequest, items: Vec<Product>) -> XResult<()> {
    let m = Marketplace::try_from(scan.marketplace_id as i16).map_err(XError::Other)?;
    let builder = XlsxBuilder::new(path, m, r, items, scan)?;

    info!("Building xlsx");
    builder
        .build_products_sheet()?
        .build_errors_sheet()?
        .build_options_sheet()?
        .close()?;

    Ok(())
}
