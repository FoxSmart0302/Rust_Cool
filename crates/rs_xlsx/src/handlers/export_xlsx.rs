use crate::app::State;
use crate::error::{XError, XResult};
use crate::generate;
use crate::inbound_eligibility::get_from_redis;
use crate::populate::populate;
use crate::product::Product;
use crate::result_request::{RawResultRequest, ResultRequest};
use crate::results::{ResultsQuery, TableType};
use crate::source::RemoteGzippedJsonStore;
use actix_files::NamedFile;
use actix_request_identifier::RequestId;
use actix_web::http::header::ContentDisposition;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};
use redis::aio::ConnectionManager;
use rs_models::{Note, Repo, Scan};
use std::collections::HashMap;
use tempfile::Builder;
use tracing::{info, span};
use tracing::{instrument, Level};

// @todo guard
#[post("/scan/{id}/export/xlsx")]
pub async fn export_xlsx(
    path: web::Path<i64>,
    req: Json<RawResultRequest>,
    server: web::Data<State>,
    r: HttpRequest,
    req_id: RequestId,
) -> XResult<NamedFile> {
    println!("req id {}", req_id.as_str());
    let scan_id = path.into_inner();
    let _span = span!(
        Level::INFO,
        "export_xlsx",
        req_id = req_id.as_str(),
        scan_id
    )
    .entered();
    info!("raw req {}", serde_json::to_string(&req)?);

    let scan = server.repo.scans.find(scan_id).await?;
    let req = req.into_inner().parse(&scan)?;
    info!("parsed req {}", serde_json::to_string(&req)?);

    // Load user via auth
    let user_id = server.auth_user(&r).await?;
    let user = server.repo.users.find(user_id).await?;

    // Make sure user has access to scan
    let can_access = user.account_id == scan.account_id || user.is_admin();
    if !can_access {
        return Err(XError::Unauthorized);
    }
    let _span = span!(Level::INFO, "authed", user_id, account_id = user.account_id,).entered();

    _export_xlsx(server, scan, req).await
}

#[instrument(err(Debug), skip(server, scan, req))]
async fn _export_xlsx(
    server: web::Data<State>,
    scan: Scan,
    req: ResultRequest,
) -> XResult<NamedFile> {
    // 1. Save if dirty
    server.save_if_dirty(&scan).await?;

    // 2. Load products from PG
    // First populate the table, if needed
    let source = RemoteGzippedJsonStore::new(scan.results.clone());
    populate(server.db.clone(), &scan, source, &server.usage_tracker).await?;

    // Then, perform the query.
    let mut products = server
        .repo
        .get_results(&scan, &req, TableType::Products)
        .await?;
    let products_count = products.len();
    info!("products count {}", products_count);
    server
        .repo
        .get_results(&scan, &req, TableType::Errors)
        .await?
        .into_iter()
        .for_each(|x| products.push(x));
    info!("errors count {}", products.len() - products_count);

    // 3. Add notes and inbound eligibility data
    products.add_notes(&server.repo, &scan).await?;
    products
        .add_inbound_eligibility(server.redis.clone(), &scan)
        .await?;

    // 4. Return XLSX
    let path_guard = Builder::new().suffix(".xlsx").tempfile()?.into_temp_path();
    let p = path_guard.as_os_str().to_str().unwrap();
    info!("tmp path {}", p);

    generate(scan, p, req, products)?;

    info!("Success");

    Ok(NamedFile::open(p)?.set_content_disposition(ContentDisposition::attachment("report.xlsx")))
}

#[async_trait::async_trait]
trait AddNotes {
    async fn add_notes(&mut self, repo: &Repo, scan: &Scan) -> XResult<()>;
}

#[async_trait::async_trait]
impl AddNotes for Vec<Product> {
    async fn add_notes(&mut self, repo: &Repo, scan: &Scan) -> XResult<()> {
        let notes: HashMap<String, Note> = repo
            .notes
            .all_for(scan.marketplace_id as i16, scan.account_id)
            .await?
            .into_iter()
            .map(|x| (x.asin.clone(), x))
            .collect();

        self.iter_mut().for_each(|x| {
            if x.asin.is_none() {
                return;
            }

            if let Some(note) = notes.get(x.asin.as_ref().unwrap()) {
                x.note = Some(note.note.clone());
            }
        });

        Ok(())
    }
}

#[async_trait::async_trait]
trait AddInboundEligibility {
    async fn add_inbound_eligibility(
        &mut self,
        redis: ConnectionManager,
        scan: &Scan,
    ) -> XResult<()>;
}

#[async_trait::async_trait]
impl AddInboundEligibility for Vec<Product> {
    async fn add_inbound_eligibility(
        &mut self,
        redis: ConnectionManager,
        scan: &Scan,
    ) -> XResult<()> {
        let chunk_size = 1000;
        let mut ranges = vec![];
        let mut start = 0;
        loop {
            let end = start + chunk_size;
            if end > self.len() {
                ranges.push((start, self.len()));
                break;
            } else {
                ranges.push((start, end));
                start = end;
            }
        }

        for range in ranges {
            // Build a list of asins
            let mut asins = Vec::with_capacity(chunk_size);
            for product in &self[range.0..range.1] {
                asins.push(product.asin.as_ref().map_or("", |x| x.as_str()));
            }

            // Pull inbound eligibility for those asins
            let inbound_eligibility = get_from_redis(
                redis.clone(),
                scan.account_id,
                scan.marketplace_id as i16,
                &asins,
            )
            .await?;

            // Add inbound eligibility to products
            for (i, product) in self[range.0..range.1].iter_mut().enumerate() {
                product.inbound_eligibility = inbound_eligibility[i].clone();
            }
        }

        Ok(())
    }
}
