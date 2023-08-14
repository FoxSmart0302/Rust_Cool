use crate::error::XResult;
use crate::selling_partner_key::SellingPartnerKey;
use crate::simple_repo;
use sqlx::{Pool, Postgres};

simple_repo!(SellingPartnerKeyRepo);

impl SellingPartnerKeyRepo {
    pub async fn all_valid(&self) -> XResult<Vec<SellingPartnerKey>> {
        let res = sqlx::query_as::<_, SellingPartnerKey>(
            "SELECT * from selling_partner_keys where is_valid = true",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(res)
    }
}
