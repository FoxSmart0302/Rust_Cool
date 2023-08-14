use crate::error::XResult;
use crate::selling_partner_key_marketplaces::SellingPartnerKeyMarketplaces;
use crate::simple_repo;
use sqlx::{Pool, Postgres};

simple_repo!(SellingPartnerKeyMarketplaceRepo);

impl SellingPartnerKeyMarketplaceRepo {
    pub async fn all(&self) -> XResult<Vec<SellingPartnerKeyMarketplaces>> {
        let res = sqlx::query_as::<_, SellingPartnerKeyMarketplaces>(
            "SELECT * from selling_partner_key_marketplaces",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(res)
    }
}
