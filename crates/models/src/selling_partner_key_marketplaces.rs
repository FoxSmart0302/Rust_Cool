use chrono::NaiveDateTime;
use marketplaces::Marketplace;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct SellingPartnerKeyMarketplaces {
    pub id: i64,
    pub selling_partner_key_id: i64,
    #[sqlx(try_from = "i64")]
    pub marketplace_id: Marketplace,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
