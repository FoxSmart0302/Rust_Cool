use crate::handlers::enrich_pricing::enrich_pricing_result::PricingResult;
use rustc_hash::FxHashMap;
use sp_api::get_competitive_pricing::GetPricingResponse;

pub fn convert(res: GetPricingResponse) -> FxHashMap<String, PricingResult> {
    FxHashMap::from_iter(
        res.payload
            .unwrap_or_default()
            .into_iter()
            .map(|x| (x.asin.clone().unwrap_or_default(), PricingResult::from(x))),
    )
}
