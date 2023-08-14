use crate::error::XError;
use serde::Deserialize;
use sp_api::marketplaces::Marketplace;

#[derive(Deserialize)]
pub struct RawPricingReq {
    marketplace_id: i16,
    /// A comma separated list of asins
    asins: String,
}

pub struct PricingReq {
    pub marketplace: Marketplace,
    pub asins: Vec<String>,
}

impl TryFrom<RawPricingReq> for PricingReq {
    type Error = XError;

    fn try_from(value: RawPricingReq) -> Result<Self, Self::Error> {
        let m = Marketplace::try_from(value.marketplace_id).map_err(|_| {
            XError::Other(format!("Invalid marketplace id {}", value.marketplace_id))
        })?;

        let asins = value
            .asins
            .split(',')
            .map(|x| x.trim().to_string())
            .collect::<Vec<String>>();

        if asins.is_empty() {
            return Err(XError::Other("No asins provided".to_string()));
        }
        if asins.len() > 20 {
            return Err(XError::Other(
                "Too many asins provided. Max is 20".to_string(),
            ));
        }

        Ok(PricingReq {
            marketplace: m,
            asins,
        })
    }
}
