use crate::env;
use crate::error::XResult;
use chrono::{Duration, Utc};
use dashmap::DashMap;
use sp_api::marketplaces::SpApiRegion;
use sp_api::selling_partner::{STSCredentials, SellingPartner};
use std::ops::Sub;
use strum::IntoEnumIterator;

/// Provides a centralized source to retrieve recently
/// refreshed STS credentials from.
///
/// This will automatically refresh STS credentials
/// before a [get](STSFactory::get) request if the expiration time is
/// within the next 5 minutes.
// @todo want to refactor out all `refresh_sts` comments to use this instead
pub struct STSFactory {
    creds: DashMap<SpApiRegion, STSCredentials>,
}

impl STSFactory {
    pub async fn new() -> XResult<Self> {
        let creds = DashMap::new();

        for region in SpApiRegion::iter() {
            let sts = SellingPartner::refresh_sts(
                region,
                env("IAM_ACCESS_KEY"),
                env("IAM_SECRET_KEY"),
                env("ROLE_ARN"),
            )
            .await?;

            creds.insert(region, sts);
        }

        Ok(Self { creds })
    }

    /// Retrieve the STS credentials for the given region.
    ///
    /// If the STS credentials will expire in the next 5 minutes, it
    /// will refresh them first.
    pub async fn get(&self, region: SpApiRegion) -> XResult<STSCredentials> {
        let mut sts = self.creds.get(&region).unwrap().clone();

        // If the STS credentials expire in the next 5 minutes, refresh them first
        if Utc::now() >= sts.expiry.sub(Duration::minutes(5)) {
            sts = SellingPartner::refresh_sts(
                region,
                env("IAM_ACCESS_KEY"),
                env("IAM_SECRET_KEY"),
                env("ROLE_ARN"),
            )
            .await?;

            self.creds.insert(region, sts.clone());
        }

        Ok(sts)
    }
}
