mod enrich_pricing_request;
mod enrich_pricing_result;
mod enrich_pricing_util;

use crate::app::State;
use crate::error::XResult;
use crate::handlers::enrich_pricing::enrich_pricing_request::{PricingReq, RawPricingReq};
use crate::handlers::enrich_pricing::enrich_pricing_result::PricingResult;
use crate::handlers::enrich_pricing::enrich_pricing_util::convert;
use crate::middleware::api_key_middleware;
use crate::new_key_cache::APICall::GetCompetitivePricing;
use crate::new_key_cache::NewKeyCache;
use actix_web::web::Json;
use actix_web::{get, web};
use async_recursion::async_recursion;
use rustc_hash::FxHashMap;
use sp_api::error::SPAPIError;
use sp_api::get_competitive_pricing::{CompetitivePriceType, GetPricingResponse, ItemType};
use sp_api::marketplaces::Marketplace;
use std::sync::Arc;

#[get("/enrich/pricing", guard = "api_key_middleware")]
pub async fn enrich_pricing(
    query: web::Query<RawPricingReq>,
    server: web::Data<State>,
    // r: HttpRequest,
    // req_id: RequestId,
) -> XResult<Json<FxHashMap<String, PricingResult>>> {
    let req: PricingReq = query.into_inner().try_into()?;

    let pricing = get_pricing(server.key_cache.clone(), req.marketplace, &req.asins).await?;
    let result = convert(pricing);

    Ok(Json(result))
}

#[async_recursion]
async fn get_pricing(
    keys: Arc<NewKeyCache>,
    marketplace: Marketplace,
    asins: &[String],
) -> XResult<GetPricingResponse> {
    let (id, sp) = keys.get(marketplace, GetCompetitivePricing, 1).await?;

    match sp
        .get_competitive_pricing(marketplace, asins, ItemType::Asin)
        .await
    {
        Ok(r) => Ok(r),
        Err(SPAPIError::Amazon(e)) if e.status_code.as_u16() == 403 => {
            // Remove the invalid key
            println!("403 error for key {}. Removing from pool", id);
            keys.remove(id);

            // And retry
            get_pricing(keys, marketplace, asins).await
        }
        Err(SPAPIError::Amazon(e)) if e.status_code.as_u16() == 429 => {
            // Remove the invalid key
            println!("429 error for key {}", id);

            // And retry
            get_pricing(keys, marketplace, asins).await
        }
        Err(e) => {
            println!("Unhandled other error using key id: {}", id);
            println!("Unhandled error: {:?}", e);
            // panic!("{:?}", e);
            // @todo don't panic
            Err(e.into())
        }
    }
}

/// Returns the price of the buybox for the given ASIN.
///
/// If the landed price is available, that will be used.
/// Otherwise, the listing price + shipping - points will be used.
///
/// If the buybox price is not available, then None will be returned.
fn extract_price(p: &CompetitivePriceType) -> Option<i32> {
    if let Some(price) = p.price.landed_price.as_ref() {
        // Try to preferentially use the landed price
        Some((price.amount * 100.0).round() as i32)
    } else {
        // If the landed price does not exist then use
        // listing price + shipping - points

        let mut out = p.price.listing_price.amount;
        let shipping = p.price.shipping.as_ref().map(|x| x.amount);
        let points = p
            .price
            .points
            .as_ref()
            .and_then(|x| x.points_monetary_value.as_ref().map(|x| x.amount));

        if let Some(p) = shipping {
            out += p;
        }
        if let Some(p) = points {
            out -= p;
        }

        Some((out * 100.0).round() as i32)
    }
}
