use chrono::Utc;
use sqlx::FromRow;
use std::ops::Sub;

#[derive(Debug, Default, Clone, FromRow)]
pub struct SellingPartnerKey {
    pub id: i64,
    pub user_id: i64,
    pub account_id: i64,
    pub seller_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub region: String,
    pub is_valid: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    #[sqlx(default)]
    pub access_token_expiry: Option<chrono::DateTime<Utc>>,
}

impl SellingPartnerKey {
    // @todo
    // pub fn to_config(&self, m: Marketplace) -> Config {
    //     Config {
    //         sp_api_url: m.sp_api_url(),
    //         region: m.sp_api_region(),
    //         access_token: self.access_token.clone(),
    //         access_token_expiry: self.access_token_expiry.clone().unwrap_or_default(),
    //     }
    // }

    /// Returns true if there is no access_token_expiry or
    /// if the access token expires in the next 5 minutes.
    pub fn access_token_needs_refresh(&self) -> bool {
        self.access_token_expiry.is_none()
            || Utc::now()
                >= self
                    .access_token_expiry
                    .as_ref()
                    .unwrap()
                    .sub(chrono::Duration::minutes(5))
    }
}
