use crate::env;
use crate::error::{XError, XResult};
use dashmap::DashMap;
use rs_models::SellingPartnerKey;
use sp_api::amazon_error::AmazonErrorKind;
use sp_api::error::SPAPIError;
use sp_api::selling_partner::SellingPartner;

// @todo to prevent thundering herd, we'll want to have something periodically
// looking for nearly expired keys or STS credentials

/// Provides a centralized source for Selling Partner Key credentials.
///
/// It will automatically refresh access tokens before a get request
/// if the expiration time is within the next 5 minutes.
pub struct SellingPartnerKeyFactory {
    creds: DashMap<i64, SellingPartnerKey>,
}

impl SellingPartnerKeyFactory {
    pub fn new(keys: Vec<SellingPartnerKey>) -> Self {
        let creds = DashMap::new();

        for key in keys {
            if !key.is_valid {
                continue;
            }

            creds.insert(key.id, key);
        }

        Self { creds }
    }

    /// Returns a SellingPartnerKey for the given id, or None if the key is not found.
    ///
    /// Automatically refreshes access credentials that are about to expire.
    pub async fn get(&self, id: i64) -> XResult<Option<SellingPartnerKey>> {
        if self.creds.is_empty() {
            return Err(XError::NoKeys);
        }

        let mut keys = match self.creds.get(&id) {
            None => {
                return Ok(None);
            }
            Some(v) => v.clone(),
        };

        // If the access token expires in the next 5 minutes, refresh it first
        if keys.access_token_needs_refresh() {
            let (access_token, expiry) = match SellingPartner::refresh_access_token(
                env("SP_CLIENT_ID"),
                env("SP_SECRET"),
                keys.refresh_token.clone(),
            )
            .await
            {
                Ok(r) => r,
                Err(SPAPIError::Amazon(e)) if e.kind == AmazonErrorKind::InvalidGrant => {
                    // Invalid key, remove from pool and return none
                    self.creds.remove(&id);
                    // @todo remove from throttle pool
                    return Ok(None);
                }
                Err(e) => {
                    return Err(XError::SPAPI(e));
                }
            };

            // Update the keys and put them back in the pool
            keys.access_token = access_token;
            keys.access_token_expiry = Some(expiry);
            self.creds.insert(id, keys.clone());
        }

        Ok(Some(keys))
    }

    pub fn remove(&self, id: i64) {
        self.creds.remove(&id);
    }
}
