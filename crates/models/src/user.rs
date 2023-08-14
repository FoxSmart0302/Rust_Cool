use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub account_id: i64,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub remember_token: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_status: Option<String>,
    pub subscription_plan: Option<String>,
}

impl User {
    pub fn is_admin(&self) -> bool {
        matches!(self.email.as_str(), "tmyers273@gmail.com")
    }
}
