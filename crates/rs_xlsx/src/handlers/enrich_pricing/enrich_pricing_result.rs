use crate::handlers::enrich_pricing::extract_price;
use serde::Serialize;
use sp_api::get_competitive_pricing::{
    Condition, OfferListingCountCondition, Price, Product, SalesRank,
};

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum PricingResult {
    Success(PricingResults),
    Error { kind: String },
}

impl From<Price> for PricingResult {
    fn from(value: Price) -> Self {
        match value.status.as_str() {
            "Success" => PricingResult::Success(value.product.unwrap().into()),
            "ClientError" => PricingResult::Error { kind: value.status },
            _ => {
                panic!("Unexpected status: {}", value.status);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PricingResults {
    /// The total number of offers for this ASIN, in any condition
    total_offers: i32,
    /// The number of offers for this ASIN, in New condition
    new_offers: i32,
    /// The number of offers for this ASIN, in Used condition
    used_offers: i32,
    /// The price of the buybox, if available
    buybox_price: Option<i32>,
    /// The price of the buybox, if available, and in New condition.
    /// This should be the same as the buybox_price
    buybox_price_new: Option<i32>,
    /// The price of the buybox, if available, and in Used condition
    buybox_price_used: Option<i32>,
    /// A list of sales rank and categories for this ASIN
    sales_ranks: Vec<SalesRankWithRaw>,
}

impl From<Product> for PricingResults {
    fn from(value: Product) -> Self {
        let mut out = PricingResults {
            total_offers: 0,
            new_offers: 0,
            used_offers: 0,
            buybox_price: None,
            buybox_price_new: None,
            buybox_price_used: None,
            sales_ranks: value
                .sales_rankings
                .map(|x| x.into_iter().map(|x| x.into()).collect())
                .unwrap_or_default(),
        };

        if let Some(p) = value.competitive_pricing {
            // Extract the offer counts
            for offers in p.number_of_offer_listings {
                use OfferListingCountCondition::*;

                match offers.condition {
                    New => out.new_offers = offers.count,
                    Used => out.used_offers = offers.count,
                    Any => out.total_offers = offers.count,
                    Collectible | Refurbished | Club => {}
                }
            }

            // Find the min competitive price that is New
            out.buybox_price_new = p
                .competitive_prices
                .iter()
                .filter(|x| x.condition == Some(Condition::New))
                .filter_map(extract_price)
                .min();

            // Find the min competitive price that is Used
            out.buybox_price_new = p
                .competitive_prices
                .iter()
                .filter(|x| x.condition == Some(Condition::Used))
                .filter_map(extract_price)
                .min();

            out.buybox_price = out.buybox_price_new;
        }

        out
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SalesRankWithRaw {
    pub rank: i32,
    pub raw_title: String,
}

impl From<SalesRank> for SalesRankWithRaw {
    fn from(value: SalesRank) -> Self {
        Self {
            rank: value.rank,
            raw_title: value.product_category_id,
        }
    }
}
