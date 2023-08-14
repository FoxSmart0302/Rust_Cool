use crate::error::XResult;
use crate::new_key_cache::key_cache::KeyCache;
use crate::new_key_cache::sts_factory::STSFactory;
use sp_api::config::Config;
use sp_api::marketplaces::Marketplace;
use sp_api::selling_partner::SellingPartner;

use crate::new_key_cache::key_factory::SellingPartnerKeyFactory;
pub use key_cache::api_call::APICall;
use rs_models::{Repo, SellingPartnerKey};

mod key_cache;
mod key_factory;
mod key_limiter;
mod sts_factory;

/// Every X requests, randomize the order of the keys. This should
/// help to prevent any sort of call patterns from emerging.
const SHUFFLE_EVERY: usize = 10;

pub struct NewKeyCache {
    key_cache: KeyCache,
    sts_factory: STSFactory,
}

impl NewKeyCache {
    pub fn new(key_cache: KeyCache, sts_factory: STSFactory) -> Self {
        NewKeyCache {
            key_cache,
            sts_factory,
        }
    }

    pub async fn from_repo(repo: Repo) -> XResult<NewKeyCache> {
        let keys = repo.keys.all_valid().await?;
        let key_marketplaces = repo.key_marketplaces.all().await?;

        println!("Loaded {} keys", keys.len());
        println!("Loaded {} key_marketplaces", key_marketplaces.len());

        let key_factory = SellingPartnerKeyFactory::new(keys.clone());
        let key_cache = KeyCache::new(key_factory, &keys, &key_marketplaces).await?;
        let sts_factory = STSFactory::new().await?;

        Ok(NewKeyCache::new(key_cache, sts_factory))
    }

    /// Constructs a SellingPartner struct using the first key available
    /// for the given APICall with n tokens available.
    ///
    /// Blocks indefinitely until a key is ready.
    /// Automatically refreshes STS and access credentials that are about to expire.
    ///
    /// Returns the id of the key used and the SellingPartner struct.
    pub async fn get(
        &self,
        marketplace: Marketplace,
        api_call: APICall,
        n: usize,
    ) -> XResult<(i64, SellingPartner)> {
        let key = self.key_cache.get(marketplace, api_call, n).await?;
        let sts = self.sts_factory.get(marketplace.sp_api_region()).await?;
        let sp = SellingPartner::new(key.to_config(marketplace), sts).await?;

        Ok((key.id, sp))
    }

    pub fn remove(&self, key_id: i64) {
        self.key_cache.remove(key_id)
    }
}

trait ToConfig {
    fn to_config(&self, marketplace: Marketplace) -> Config;
}

impl ToConfig for SellingPartnerKey {
    fn to_config(&self, m: Marketplace) -> Config {
        Config {
            sp_api_url: m.sp_api_url(),
            region: m.sp_api_region(),
            access_token: self.access_token.clone(),
            access_token_expiry: self.access_token_expiry.unwrap_or_default(),
        }
    }
}
