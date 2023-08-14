pub mod small_and_light_eligibility_reasons;

use crate::inbound_eligibility::InboundEligibility;
use crate::product::small_and_light_eligibility_reasons::SmallAndLightEligibilityReasons;
#[cfg(test)]
use crate::xlsx::marketplace_helper::MyFaker;
#[cfg(test)]
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use sp_api::marketplaces::Marketplace;
use sqlx::types::Json;
use sqlx::FromRow;

/// This is the actual product that we store in the database
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct FlatProduct {
    pub id: i64,
    pub flags: Json<Option<Vec<String>>>,
    pub errors: Json<Option<Vec<String>>>,
    pub input_identifier: Option<String>,
    pub input_cost: Option<i32>,
    pub input_stock: Option<i32>,
    pub input_map: Option<i32>,
    pub input_supplier_title: Option<String>,
    pub input_supplier_sku: Option<String>,
    pub input_supplier_image: Option<String>,
    pub input_supplier_pack_quantity: Option<i32>,
    pub input_discount_per_product: Option<i32>,
    pub input_discount_supplier: Option<i32>,
    pub input_discount_cost: Option<i32>,
    pub input_total_cogs: Option<i32>,
    pub input_custom: Json<Option<Vec<String>>>,
    pub asin: Option<String>,
    pub offers: Json<Offers>,
    pub offers_total: Option<i32>,
    pub offers_list_price: Option<i32>,
    pub images: Json<Option<Vec<ImageByMarketplace>>>,
    pub amazon_title: Option<String>,
    pub is_top_level_category: Option<bool>,
    pub category_raw: Option<String>,
    pub category: Option<String>,
    pub rank: Option<i32>,
    pub buybox_price: Option<i32>,
    pub amazon_pack_quantity: Option<i32>,
    pub number_of_variations: Option<i32>,
    pub variations_list: Json<Option<Vec<String>>>,
    pub parent_asin: Option<String>,
    pub sales_ranks: Json<Option<Vec<SalesRank>>>,
    pub package_length: Option<f64>,
    pub package_width: Option<f64>,
    pub package_height: Option<f64>,
    pub package_weight: Option<f64>,
    pub package_length_unit: Option<String>,
    pub package_width_unit: Option<String>,
    pub package_height_unit: Option<String>,
    pub package_weight_unit: Option<String>,
    pub item_length: Option<f64>,
    pub item_width: Option<f64>,
    pub item_height: Option<f64>,
    pub item_weight: Option<f64>,
    pub item_length_unit: Option<String>,
    pub item_width_unit: Option<String>,
    pub item_height_unit: Option<String>,
    pub item_weight_unit: Option<String>,
    // pub dimensions: Dimensions,
    pub amazon_fees_per_item_fee: Option<i32>,
    pub amazon_fees_fba_fees: Option<i32>,
    pub amazon_fees_variable_closing_fee: Option<i32>,
    pub amazon_fees_referral_fee: Option<i32>,
    pub amazon_fees_error: Option<String>,
    // pub amazon_fees: AmazonFees,
    pub competitive_sellers: Option<i32>,
    pub brand: Option<String>,
    pub color: Option<String>,
    pub size_name: Option<String>,
    pub listing_restrictions: Json<Option<Vec<ListingRestriction>>>,
    pub financials_inbound_shipping: Option<i32>,
    pub financials_prep_cost: Option<i32>,
    pub financials_fba_storage_fees: Option<i32>,
    pub financials_net_revenue: Option<i32>,
    pub financials_profit: Option<i32>,
    pub financials_sale_proceeds: Option<i32>,
    pub financials_margin: Option<i32>,
    pub financials_roi: Option<i32>,
    // pub financials: Financials,
    pub size_tier: Option<String>,
    pub lowest_price_new_fba: Option<i32>,
    pub lowest_price_used_fba: Option<i32>,
    pub lowest_price_new_fbm: Option<i32>,
    pub lowest_price_used_fbm: Option<i32>,
    pub buybox_price_new: Option<i32>,
    pub buybox_price_used: Option<i32>,
    pub total_offers_count: Option<i32>,
    pub is_brand_blocklisted: bool,
    pub new_fba_offers_count: Option<i32>,
    pub new_fbm_offers_count: Option<i32>,
    pub is_adult: bool,
    pub is_hazmat: bool,
    pub is_meltable: bool,
    pub small_and_light_eligible: bool,
    #[sqlx(try_from = "i16")]
    pub small_and_light_eligible_reasons: SmallAndLightEligibilityReasons,
    // In hundredths of a percent, so 20.12% -> 2012
    // Calculated as BSR / total_items_in_category * 10000
    pub bsr_percentage: Option<i32>,
}

impl From<FlatProduct> for Product {
    fn from(value: FlatProduct) -> Self {
        Self {
            id: value.id,
            flags: value.flags,
            errors: value.errors,
            inputs: Inputs {
                identifier: value.input_identifier,
                cost: value.input_cost,
                stock: value.input_stock,
                map: value.input_map,
                supplier_title: value.input_supplier_title,
                supplier_sku: value.input_supplier_sku,
                supplier_image: value.input_supplier_image,
                supplier_pack_quantity: value.input_supplier_pack_quantity,
                discount_per_product: value.input_discount_per_product,
                discount_supplier: value.input_discount_supplier,
                discount_cost: value.input_discount_cost,
                total_cogs: value.input_total_cogs,
                custom_columns: value.input_custom.0,
            },
            asin: value.asin,
            offers: value.offers.0,
            images: value.images.0,
            amazon_title: value.amazon_title,
            is_top_level_category: value.is_top_level_category,
            category_raw: value.category_raw,
            category: value.category,
            rank: value.rank,
            buybox_price: value.buybox_price,
            amazon_pack_quantity: value.amazon_pack_quantity,
            number_of_variations: value.number_of_variations,
            variations_list: value.variations_list.0,
            parent_asin: value.parent_asin,
            sales_ranks: value.sales_ranks.0,
            dimensions: Dimensions {
                package_dimensions: DimensionSet {
                    length: value.package_length,
                    width: value.package_width,
                    height: value.package_height,
                    weight: value.package_weight,
                    length_unit: value.package_length_unit,
                    width_unit: value.package_width_unit,
                    height_unit: value.package_height_unit,
                    weight_unit: value.package_weight_unit,
                },
                item_dimensions: DimensionSet {
                    length: value.item_length,
                    width: value.item_width,
                    height: value.item_height,
                    weight: value.item_weight,
                    length_unit: value.item_length_unit,
                    width_unit: value.item_width_unit,
                    height_unit: value.item_height_unit,
                    weight_unit: value.item_weight_unit,
                },
            },
            amazon_fees: AmazonFees {
                per_item_fee: value.amazon_fees_per_item_fee,
                fba_fees: value.amazon_fees_fba_fees,
                variable_closing_fee: value.amazon_fees_variable_closing_fee,
                referral_fee: value.amazon_fees_referral_fee,
                error: value.amazon_fees_error,
            },
            competitive_sellers: value.competitive_sellers,
            brand: value.brand,
            color: value.color,
            size_name: value.size_name,
            listing_restrictions: value.listing_restrictions.0,
            financials: Financials {
                inbound_shipping: value.financials_inbound_shipping,
                prep_cost: value.financials_prep_cost,
                fba_storage_fees: value.financials_fba_storage_fees,
                net_revenue: value.financials_net_revenue,
                profit: value.financials_profit,
                margin: value.financials_margin,
                roi: value.financials_roi,
            },
            size_tier: value.size_tier,
            lowest_price_new_fba: value.lowest_price_new_fba,
            lowest_price_used_fba: value.lowest_price_used_fba,
            lowest_price_new_fbm: value.lowest_price_new_fbm,
            lowest_price_used_fbm: value.lowest_price_used_fbm,
            buybox_price_new: value.buybox_price_new,
            buybox_price_used: value.buybox_price_used,
            total_offers_count: value.total_offers_count,
            is_brand_blocklisted: value.is_brand_blocklisted,
            new_fba_offers_count: value.new_fba_offers_count,
            new_fbm_offers_count: value.new_fbm_offers_count,
            is_adult: value.is_adult,
            is_hazmat: value.is_hazmat,
            is_meltable: value.is_meltable,
            small_and_light_eligible: value.small_and_light_eligible,
            small_and_light_eligible_reasons: value.small_and_light_eligible_reasons,
            inbound_eligibility: None,
            note: None,
            bsr_percentage: value.bsr_percentage,
            marketplace_id: None,
        }
    }
}

impl From<Product> for FlatProduct {
    fn from(value: Product) -> Self {
        Self {
            id: value.id,
            flags: value.flags,
            errors: value.errors,
            input_identifier: value.inputs.identifier,
            input_cost: value.inputs.cost,
            input_stock: value.inputs.stock,
            input_map: value.inputs.map,
            input_supplier_title: value.inputs.supplier_title,
            input_supplier_sku: value.inputs.supplier_sku,
            input_supplier_image: value.inputs.supplier_image,
            input_supplier_pack_quantity: value.inputs.supplier_pack_quantity,
            input_discount_per_product: value.inputs.discount_per_product,
            input_discount_supplier: value.inputs.discount_supplier,
            input_discount_cost: value.inputs.discount_cost,
            input_total_cogs: value.inputs.total_cogs,
            input_custom: Json(value.inputs.custom_columns),
            asin: value.asin,
            offers_list_price: value.offers.list_price,
            offers: Json(value.offers),
            offers_total: value.total_offers_count,
            images: Json(value.images),
            amazon_title: value.amazon_title,
            is_top_level_category: value.is_top_level_category,
            category_raw: value.category_raw,
            category: value.category,
            rank: value.rank,
            buybox_price: value.buybox_price,
            amazon_pack_quantity: value.amazon_pack_quantity,
            number_of_variations: value.number_of_variations,
            variations_list: Json(value.variations_list),
            parent_asin: value.parent_asin,
            sales_ranks: Json(value.sales_ranks),
            package_length: value.dimensions.package_dimensions.length,
            package_width: value.dimensions.package_dimensions.width,
            package_height: value.dimensions.package_dimensions.height,
            package_weight: value.dimensions.package_dimensions.weight,
            package_length_unit: value.dimensions.package_dimensions.length_unit,
            package_width_unit: value.dimensions.package_dimensions.width_unit,
            package_height_unit: value.dimensions.package_dimensions.height_unit,
            package_weight_unit: value.dimensions.package_dimensions.weight_unit,
            item_length: value.dimensions.item_dimensions.length,
            item_width: value.dimensions.item_dimensions.width,
            item_height: value.dimensions.item_dimensions.height,
            item_weight: value.dimensions.item_dimensions.weight,
            item_length_unit: value.dimensions.item_dimensions.length_unit,
            item_width_unit: value.dimensions.item_dimensions.width_unit,
            item_height_unit: value.dimensions.item_dimensions.height_unit,
            item_weight_unit: value.dimensions.item_dimensions.weight_unit,
            amazon_fees_per_item_fee: value.amazon_fees.per_item_fee,
            amazon_fees_fba_fees: value.amazon_fees.fba_fees,
            amazon_fees_variable_closing_fee: value.amazon_fees.variable_closing_fee,
            amazon_fees_referral_fee: value.amazon_fees.referral_fee,
            amazon_fees_error: value.amazon_fees.error,
            competitive_sellers: value.competitive_sellers,
            brand: value.brand,
            color: value.color,
            size_name: value.size_name,
            listing_restrictions: Json(value.listing_restrictions),
            financials_inbound_shipping: value.financials.inbound_shipping,
            financials_prep_cost: value.financials.prep_cost,
            financials_fba_storage_fees: value.financials.fba_storage_fees,
            financials_net_revenue: value.financials.net_revenue,
            financials_profit: value.financials.profit,
            financials_sale_proceeds: value.financials.net_revenue,
            financials_margin: value.financials.margin,
            financials_roi: value.financials.roi,
            size_tier: value.size_tier,
            lowest_price_new_fba: value.lowest_price_new_fba,
            lowest_price_used_fba: value.lowest_price_used_fba,
            lowest_price_new_fbm: value.lowest_price_new_fbm,
            lowest_price_used_fbm: value.lowest_price_used_fbm,
            buybox_price_new: value.buybox_price_new,
            buybox_price_used: value.buybox_price_used,
            total_offers_count: value.total_offers_count,
            is_brand_blocklisted: value.is_brand_blocklisted,
            new_fba_offers_count: value.new_fba_offers_count,
            new_fbm_offers_count: value.new_fbm_offers_count,
            is_adult: value.is_adult,
            is_hazmat: value.is_hazmat,
            is_meltable: value.is_meltable,
            small_and_light_eligible: value.small_and_light_eligible,
            small_and_light_eligible_reasons: value.small_and_light_eligible_reasons,
            bsr_percentage: value.bsr_percentage,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
pub struct Product {
    pub id: i64,
    pub flags: Json<Option<Vec<String>>>,
    pub errors: Json<Option<Vec<String>>>,
    pub inputs: Inputs,
    pub asin: Option<String>,
    pub offers: Offers,
    pub images: Option<Vec<ImageByMarketplace>>,
    pub amazon_title: Option<String>,
    pub is_top_level_category: Option<bool>,
    pub category_raw: Option<String>,
    pub category: Option<String>,
    pub rank: Option<i32>,
    pub buybox_price: Option<i32>,
    pub amazon_pack_quantity: Option<i32>,
    pub number_of_variations: Option<i32>,
    pub variations_list: Option<Vec<String>>,
    pub parent_asin: Option<String>,
    pub sales_ranks: Option<Vec<SalesRank>>,
    pub dimensions: Dimensions,
    pub amazon_fees: AmazonFees,
    pub competitive_sellers: Option<i32>,
    pub brand: Option<String>,
    pub color: Option<String>,
    pub size_name: Option<String>,
    pub listing_restrictions: Option<Vec<ListingRestriction>>,
    pub financials: Financials,
    pub size_tier: Option<String>,
    pub lowest_price_new_fba: Option<i32>,
    pub lowest_price_used_fba: Option<i32>,
    pub lowest_price_new_fbm: Option<i32>,
    pub lowest_price_used_fbm: Option<i32>,
    pub buybox_price_new: Option<i32>,
    pub buybox_price_used: Option<i32>,
    pub total_offers_count: Option<i32>,
    pub is_brand_blocklisted: bool,
    pub new_fba_offers_count: Option<i32>,
    pub new_fbm_offers_count: Option<i32>,
    pub is_adult: bool,
    pub is_hazmat: bool,
    pub is_meltable: bool,
    pub small_and_light_eligible: bool,
    pub small_and_light_eligible_reasons: SmallAndLightEligibilityReasons,
    pub inbound_eligibility: Option<InboundEligibility>,
    pub note: Option<String>,
    // In hundredths of a percent, so 20.12% -> 2012
    // Calculated as BSR / total_items_in_category * 10000
    pub bsr_percentage: Option<i32>,
    pub marketplace_id: Option<Marketplace>,
}

impl Product {
    pub fn has_errors(&self) -> bool {
        match &self.errors.0 {
            Some(errors) => !errors.is_empty(),
            None => false,
        }
    }

    #[cfg(test)]
    pub fn fake() -> Self {
        let mut p = Self {
            id: Faker.fake::<u64>() as i64,
            flags: Json(Faker.fake()),
            errors: Json(Faker.fake()),
            inputs: Faker.fake(),
            asin: Faker.fake(),
            offers: Faker.fake(),
            images: Faker.fake(),
            amazon_title: Faker.fake(),
            is_top_level_category: Faker.fake(),
            category_raw: Faker.fake(),
            category: Faker.fake(),
            rank: Faker.fake(),
            buybox_price: Faker.fake(),
            amazon_pack_quantity: Faker.fake(),
            number_of_variations: Faker.fake(),
            variations_list: Faker.fake(),
            parent_asin: Faker.fake(),
            sales_ranks: Faker.fake(),
            dimensions: Faker.fake(),
            amazon_fees: Faker.fake(),
            competitive_sellers: Faker.fake(),
            brand: Faker.fake(),
            color: Faker.fake(),
            size_name: Faker.fake(),
            listing_restrictions: Faker.fake(),
            financials: Faker.fake(),
            size_tier: Faker.fake(),
            lowest_price_new_fba: Faker.fake(),
            lowest_price_used_fba: Faker.fake(),
            lowest_price_new_fbm: Faker.fake(),
            lowest_price_used_fbm: Faker.fake(),
            buybox_price_new: Faker.fake(),
            buybox_price_used: Faker.fake(),
            total_offers_count: Faker.fake(),
            is_brand_blocklisted: Faker.fake(),
            new_fba_offers_count: Faker.fake(),
            new_fbm_offers_count: Faker.fake(),
            is_adult: Faker.fake(),
            is_hazmat: Faker.fake(),
            is_meltable: Faker.fake(),
            small_and_light_eligible: Faker.fake(),
            small_and_light_eligible_reasons: Faker.fake(),
            inbound_eligibility: Faker.fake(),
            note: Faker.fake(),
            bsr_percentage: Faker.fake(),
            marketplace_id: MyFaker.fake(),
        };

        // Round f64s, as precision gets lost and makes partial equality fail
        p.dimensions.package_dimensions.length = p
            .dimensions
            .package_dimensions
            .length
            .map(|x| (x * 100.0).round() / 100.0);
        p.dimensions.package_dimensions.width = p
            .dimensions
            .package_dimensions
            .width
            .map(|x| (x * 100.0).round() / 100.0);
        p.dimensions.package_dimensions.height = p
            .dimensions
            .package_dimensions
            .height
            .map(|x| (x * 100.0).round() / 100.0);
        p.dimensions.package_dimensions.weight = p
            .dimensions
            .package_dimensions
            .weight
            .map(|x| (x * 100.0).round() / 100.0);
        p.dimensions.item_dimensions.length = p
            .dimensions
            .item_dimensions
            .length
            .map(|x| (x * 100.0).round() / 100.0);
        p.dimensions.item_dimensions.width = p
            .dimensions
            .item_dimensions
            .width
            .map(|x| (x * 100.0).round() / 100.0);
        p.dimensions.item_dimensions.height = p
            .dimensions
            .item_dimensions
            .height
            .map(|x| (x * 100.0).round() / 100.0);
        p.dimensions.item_dimensions.weight = p
            .dimensions
            .item_dimensions
            .weight
            .map(|x| (x * 100.0).round() / 100.0);

        p
    }

    #[cfg(test)]
    pub fn fake_no_extra() -> Self {
        let mut f = Self::fake();
        f.note = None;
        f.inbound_eligibility = None;
        f.marketplace_id = None;

        f
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct Inputs {
    /// Even though cost is required as a mapping field, we can
    /// still have individual rows that are missing a cost.
    pub identifier: Option<String>,
    /// Even though cost is required as a mapping field, we can
    pub cost: Option<i32>,
    pub stock: Option<i32>,
    pub map: Option<i32>,
    pub supplier_title: Option<String>,
    pub supplier_sku: Option<String>,
    pub supplier_image: Option<String>,
    pub supplier_pack_quantity: Option<i32>,
    pub discount_per_product: Option<i32>,
    pub discount_supplier: Option<i32>,
    pub discount_cost: Option<i32>,
    pub total_cogs: Option<i32>,
    pub custom_columns: Option<Vec<String>>,
}

/*
{"id":505,"inputs":{"identifier":"793831020315","cost":1195,"supplier_title":"Streetwise Mini SMACK 20,000,000* Stun Gun","supplier_sku":"CEzz-DIS-SMKM20BK","supplier_image":"https://images.inventorysource.com/images/ce/CEzz-DIS-SMKM20BK.jpg","supplier_pack_quantity":1,"discount_cost":1195,"total_cogs":2390},"asin":"B01M8M0DOO","offers":{"buybox_eligible_offers":null,"number_of_offers":null,"lowest_prices":null,"buy_box_prices":null},"images":[{"marketplace_id":"ATVPDKIKX0DER","image":[{"height":700,"width":1400,"link":"https://m.media-amazon.com/images/I/61KuS-YZsFL.jpg","variant":"MAIN"},{"height":250,"width":500,"link":"https://m.media-amazon.com/images/I/313Z4ihCwyL.jpg","variant":"MAIN"},{"height":38,"width":75,"link":"https://m.media-amazon.com/images/I/313Z4ihCwyL._SL75_.jpg","variant":"MAIN"}]}],"amazon_title":"Bundle Package Streetwise Mini SMACK Stun Gun","amazon_pack_quantity":2,"sales_ranks":[],"dimensions":{"package_dimensions":{},"item_dimensions":{}},"amazon_fees":{"referral_fee":30,"Error":""},"brand":"Home Self Defense Producst","color":"Black","listing_restrictions":null,"financials":{"prep_cost":200},"size_tier":"Small Standard","is_brand_blocklisted":false,"is_adult":false,"is_hazmat":false,"is_meltable":false,"small_and_light_eligible":true,"small_and_light_eligible_reasons":0,"inbound_eligibility":null,"note":"","bsr_percentage":null,"marketplace_id":null}
 */

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct Offers {
    pub total_offers_count: Option<i32>,
    pub list_price: Option<i32>,
    pub buybox_eligible_offers: Option<Vec<OfferCount>>,
    pub number_of_offers: Option<Vec<OfferCount>>,
    pub lowest_prices: Option<Vec<PriceInfo>>,
    pub buy_box_prices: Option<Vec<PriceInfo>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct OfferCount {
    pub condition: String,
    pub fulfillment_channel: Option<String>,
    pub offer_count: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct PriceInfo {
    pub condition: String,
    pub fulfillment_channel: Option<String>,
    pub landed_price: i32,
    pub listing_price: Option<i32>,
    pub shipping: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct ImageByMarketplace {
    pub marketplace_id: String,
    pub image: Vec<Image>,
}

pub trait ImageHelper {
    fn get_main_image(&self, marketplace: Marketplace) -> Option<String> {
        self.get_image_type(marketplace, "MAIN")
    }
    /// ImageTypeForMarketplaceId tries to return the link for the given image
    /// variant in the given marketplace. If not found, it will fall back to
    /// the first marketplace containing that variant. If the variant still
    /// can't be found, it will fall back to the first image available.
    fn get_image_type(&self, marketplace: Marketplace, variant: &str) -> Option<String>;
}

impl ImageHelper for Vec<ImageByMarketplace> {
    fn get_image_type(&self, marketplace: Marketplace, variant: &str) -> Option<String> {
        let mid = marketplace.amazon_marketplace_id();

        // Try to find by (marketplace, variant) tuple
        for i in self.iter().filter(|x| x.marketplace_id == mid) {
            if let Some(img) = i.image.iter().find(|x| x.variant == variant) {
                return Some(img.link.clone());
            }
        }

        // Try to find with just (variant)
        for i in self.iter() {
            if let Some(img) = i.image.iter().find(|x| x.variant == variant) {
                return Some(img.link.clone());
            }
        }

        // Just pull the first image
        for i in self.iter() {
            if let Some(img) = i.image.first() {
                return Some(img.link.clone());
            }
        }

        None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct Image {
    height: i32,
    width: i32,
    link: String,
    variant: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct SalesRank {
    rank: i32,
    title: Option<String>,
    raw_title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct Dimensions {
    pub package_dimensions: DimensionSet,
    pub item_dimensions: DimensionSet,
}

impl Dimensions {
    /// GetHeight returns the Height. We preferentially try to pull from the package dimensions,
    /// as that will generally be the most important factor for sellers. If the package dimension
    /// is not available, we fall back to the item dimension.
    ///
    /// A good example as to why we preferentially use package dimensions would be for an item that
    /// "unpacks" into something larger. Think about something like an inflatable swimming pool.
    ///
    /// The package dimensions (or the size of hte swimming pool in its box) will be much smaller
    /// than the item dimensions (or the size of the swimming pool when it's inflated).
    ///
    /// The seller will primarily be concerned with the package dimensions, as this will
    /// be the dimension set that matters for logistics, FBA fees, etc.
    pub fn get_height(&self) -> Option<f64> {
        match self.package_dimensions.height {
            Some(v) if v != 0.0 => return Some(v),
            _ => {}
        };

        match self.item_dimensions.height {
            Some(v) if v != 0.0 => Some(v),
            _ => None,
        }
    }

    /// Returns the width. See note in [get_height](Self::get_height)
    pub fn get_width(&self) -> Option<f64> {
        match self.package_dimensions.width {
            Some(v) if v != 0.0 => return Some(v),
            _ => {}
        };

        match self.item_dimensions.width {
            Some(v) if v != 0.0 => Some(v),
            _ => None,
        }
    }

    /// Returns the length. See note in [get_height](Self::get_height)
    pub fn get_length(&self) -> Option<f64> {
        match self.package_dimensions.length {
            Some(v) if v != 0.0 => return Some(v),
            _ => {}
        };

        match self.item_dimensions.length {
            Some(v) if v != 0.0 => Some(v),
            _ => None,
        }
    }

    /// Returns the weight. See note in [get_height](Self::get_height)
    pub fn get_weight(&self) -> Option<f64> {
        match self.package_dimensions.weight {
            Some(v) if v != 0.0 => return Some(v),
            _ => {}
        };

        match self.item_dimensions.weight {
            Some(v) if v != 0.0 => Some(v),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct DimensionSet {
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub length_unit: Option<String>,
    pub width_unit: Option<String>,
    pub height_unit: Option<String>,
    pub weight_unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct AmazonFees {
    pub per_item_fee: Option<i32>,
    pub fba_fees: Option<i32>,
    pub variable_closing_fee: Option<i32>,
    pub referral_fee: Option<i32>,
    pub error: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct ListingRestriction {
    condition_type: String,
    marketplace_id: String,
    reasons: Vec<ListingRestrictionReason>,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct ListingRestrictionReason {
    message: String,
    reason_code: String,
    links: Vec<Link>,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct Link {
    resource: String,
    title: String,
    r#type: String,
    verb: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct Financials {
    pub inbound_shipping: Option<i32>,
    pub prep_cost: Option<i32>,
    pub fba_storage_fees: Option<i32>,
    pub net_revenue: Option<i32>,
    pub profit: Option<i32>,
    pub margin: Option<i32>,
    pub roi: Option<i32>,
}

// #[derive(Debug, Serialize, Deserialize, Clone, Dummy, FromRow, PartialEq)]
// pub struct InboundEligibility {
//     pub eligible: bool,
//     pub update_at: u32,
//     pub ineligible_codes: Vec<u8>,
// }
