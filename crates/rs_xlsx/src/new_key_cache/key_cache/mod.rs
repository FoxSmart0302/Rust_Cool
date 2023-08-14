pub mod api_call;
mod throttling;

use crate::error::XResult;
use crate::new_key_cache::key_cache::api_call::APICall;
use crate::new_key_cache::key_cache::throttling::throttle_info;
use crate::new_key_cache::key_factory::SellingPartnerKeyFactory;
use crate::new_key_cache::key_limiter::KeyLimiter;
use dashmap::DashMap;
use rs_models::{SellingPartnerKey, SellingPartnerKeyMarketplaces};
use rustc_hash::FxHashSet;
use sp_api::marketplaces::Marketplace;
use std::num::NonZeroU32;
use std::time::Duration;
use strum::IntoEnumIterator;
use tokio::time::sleep;

pub struct KeyCache {
    dm: DashMap<(Marketplace, APICall), KeyLimiter>,
    key_factory: SellingPartnerKeyFactory,
}

impl KeyCache {
    pub async fn new(
        key_factory: SellingPartnerKeyFactory,
        keys: &[SellingPartnerKey],
        key_marketplaces: &[SellingPartnerKeyMarketplaces],
    ) -> XResult<KeyCache> {
        // Create a set out of the valid key ids
        let mut valid_keys = FxHashSet::default();
        for key in keys {
            if key.is_valid {
                valid_keys.insert(key.id);
            }
        }

        let out = DashMap::default();

        for api_call in APICall::iter() {
            let (requests_per_second, burst) = throttle_info(api_call);

            for marketplace in Marketplace::iter() {
                let ids = key_marketplaces
                    .iter()
                    .filter(|x| x.marketplace_id == marketplace)
                    .filter(|x| valid_keys.contains(&x.id))
                    .map(|x| x.id)
                    .collect::<Vec<i64>>();

                let chunk = KeyLimiter::new(ids, requests_per_second, burst);
                out.insert((marketplace, api_call), chunk);
            }
        }

        Ok(KeyCache {
            dm: out,
            key_factory,
        })
    }

    /// Returns the first key with n tokens available.
    ///
    /// Blocks indefinitely until a key is ready.
    pub async fn get(
        &self,
        marketplace: Marketplace,
        api_call: APICall,
        n: usize,
    ) -> XResult<SellingPartnerKey> {
        let key = loop {
            let id = self.get_first_key(marketplace, api_call, n).await;

            // A key id can be returned, but not present in the key factory due to race
            // conditions. So, we will retry if the key is not found.
            match self.key_factory.get(id).await? {
                Some(k) => break k,
                None => {
                    println!("Key not found, retrying...")
                }
            };
        };

        Ok(key)
    }

    async fn get_first_key(&self, m: Marketplace, api_call: APICall, n: usize) -> i64 {
        // Loop until we get a key with n tokens available
        let id = loop {
            // Try to pull the first key that has n tokens available
            let mut t = self.dm.get_mut(&(m, api_call)).unwrap();
            let id = t.value_mut().take(NonZeroU32::new(n as u32).unwrap());
            drop(t);

            // If there is no key with n tokens available, wait a bit and try again
            if id.is_none() {
                sleep(Duration::from_millis(50)).await;
            } else {
                break id.unwrap();
            }
        };

        id
    }

    pub fn remove(&self, key_id: i64) {
        // Remove from the dashmap
        for mut t in self.dm.iter_mut() {
            t.value_mut().remove(key_id);
        }

        self.key_factory.remove(key_id);
    }
}
