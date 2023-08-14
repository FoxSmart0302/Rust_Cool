mod table_type;

use crate::error::XResult;
use crate::product::{FlatProduct, Product};
use crate::result_request::ResultRequest;
pub use crate::results::table_type::TableType;
use composable_query_builder::{ComposableQueryBuilder, OrderDir};
use rs_models::{Repo, Scan};

#[async_trait::async_trait]
pub trait ResultsQuery {
    async fn get_results(
        &self,
        scan: &Scan,
        req: &ResultRequest,
        table_type: TableType,
    ) -> XResult<Vec<Product>>;
}

#[async_trait::async_trait]
impl ResultsQuery for Repo {
    async fn get_results(
        &self,
        scan: &Scan,
        req: &ResultRequest,
        table_type: TableType,
    ) -> XResult<Vec<Product>> {
        let has_sort = req.sort.as_ref().map(|x| !x.is_empty()).unwrap_or(false);

        let mut q = ComposableQueryBuilder::new()
            .table(table_name(scan.id, table_type))
            .limit_opt(req.limit)
            .offset_opt(req.skip);

        if has_sort {
            let sort = req.sort.as_ref().unwrap();
            let dir = match sort.starts_with('-') {
                true => (sort.trim_start_matches('-'), OrderDir::Desc),
                false => (sort.as_str(), OrderDir::Asc),
            };

            q = q.order_by(dir.0, dir.1);
        }

        for filter in &req.filter {
            q = filter.apply(q)?;
        }

        let mut q = q.into_builder();
        let q = q.build_query_as::<FlatProduct>();
        let r = q
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect();

        Ok(r)
    }
}

pub fn table_name(scan_id: i64, table_type: TableType) -> String {
    match table_type {
        TableType::Errors => format!("scan_{}_errors", scan_id),
        TableType::Products => format!("scan_{}", scan_id),
    }
}
