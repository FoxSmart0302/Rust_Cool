use crate::error::XError;
use crate::product::{ImageHelper, Product};
use crate::xlsx::display::Display;
use crate::xlsx::display::Display::OptMoney;
use crate::xlsx::formula::{
    DISCOUNT_COST, INBOUND_SHIPPING, MARGIN, NET_REVENUE, PREP_COST, PROFIT, ROI, TOTAL_COGS,
};
use rs_models::Scan;
use serde::{Deserialize, Serialize};
use sp_api::marketplaces::Marketplace;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, EnumIter)]
#[serde(try_from = "&str", into = "String", rename_all = "snake_case")]
pub enum XLSXColumn {
    Checkbox,
    Notes,
    Chart,
    Flags,
    Errors,
    Asin,
    AmazonTitle,
    FbaEligibility,
    #[serde(alias = "image")]
    Images,
    #[serde(alias = "size")]
    SizeName,
    Brand,
    Color,
    ParentAsin,
    VariationsList,
    NumberOfVariations,
    AmazonPackQuantity,
    SmallAndLightEligible,
    SmallAndLightEligibleReasons,
    BuyboxPrice,
    BuyboxPriceNew,
    BuyboxPriceUsed,
    #[serde(alias = "number_of_total_offers")]
    TotalOffersCount,
    Rank,
    Category,
    IsTopLevelCategory,
    BsrPercentage,
    Width,
    Height,
    Length,
    Weight,
    #[serde(alias = "number_of_competitive_sellers")]
    CompetitiveSellers,
    #[serde(alias = "inbound_shipping")]
    FinancialsInboundShipping,
    #[serde(alias = "prep_cost")]
    FinancialsPrepCost,
    #[serde(alias = "fba_storage_fees")]
    FinancialsFbaStorageFees,
    #[serde(alias = "per_item_fee")]
    AmazonFeesPerItemFee,
    #[serde(alias = "fba_fees")]
    AmazonFeesFbaFees,
    #[serde(alias = "variable_closing_fee")]
    AmazonFeesVariableClosingFee,
    #[serde(alias = "referral_fee")]
    AmazonFeesReferralFee,
    #[serde(alias = "net_revenue")]
    FinancialsNetRevenue,
    #[serde(alias = "profit")]
    FinancialsProfit,
    #[serde(alias = "margin")]
    FinancialsMargin,
    #[serde(alias = "roi")]
    FinancialsRoi,
    #[serde(alias = "inputs.identifier")]
    InputIdentifier,
    #[serde(alias = "inputs.cost")]
    InputCost,
    #[serde(alias = "inputs.per_product_discount")]
    InputDiscountPerProduct,
    #[serde(alias = "inputs.discount_cost")]
    InputDiscountCost,
    #[serde(alias = "inputs.total_cogs")]
    InputTotalCogs,
    #[serde(alias = "inputs.stock")]
    InputStock,
    #[serde(alias = "inputs.map")]
    InputMap,
    #[serde(alias = "inputs.supplier_title")]
    InputSupplierTitle,
    #[serde(alias = "inputs.supplier_sku")]
    InputSupplierSku,
    #[serde(alias = "inputs.supplier_image")]
    InputSupplierImage,
    #[serde(alias = "inputs.supplier_pack_quantity")]
    InputSupplierPackQuantity,
    SizeTier,
    LowestPriceNewFba,
    LowestPriceUsedFba,
    LowestPriceNewFbm,
    LowestPriceUsedFbm,
    #[serde(alias = "number_of_new_fba_offers")]
    NewFbaOffersCount,
    #[serde(alias = "number_of_new_fbm_offers")]
    NewFbmOffersCount,
    IsBrandBlocklisted,
    IsMeltable,
    AddToAmazonLink,
    InputCustom(u32),
}

impl From<XLSXColumn> for String {
    fn from(value: XLSXColumn) -> Self {
        use XLSXColumn::*;

        match value {
            Checkbox => "checkbox".to_string(),
            Notes => "notes".to_string(),
            Chart => "chart".to_string(),
            Flags => "flags".to_string(),
            Errors => "errors".to_string(),
            Asin => "asin".to_string(),
            AmazonTitle => "amazon_title".to_string(),
            FbaEligibility => "fba_eligibility".to_string(),
            Images => "images".to_string(),
            SizeName => "size_name".to_string(),
            Brand => "brand".to_string(),
            Color => "color".to_string(),
            ParentAsin => "parent_asin".to_string(),
            VariationsList => "variations_list".to_string(),
            NumberOfVariations => "number_of_variations".to_string(),
            AmazonPackQuantity => "amazon_pack_quantity".to_string(),
            SmallAndLightEligible => "small_and_light_eligible".to_string(),
            SmallAndLightEligibleReasons => "small_and_light_eligible_reasons".to_string(),
            BuyboxPrice => "buybox_price".to_string(),
            BuyboxPriceNew => "buybox_price_new".to_string(),
            BuyboxPriceUsed => "buybox_price_used".to_string(),
            TotalOffersCount => "total_offers_count".to_string(),
            Rank => "rank".to_string(),
            Category => "category".to_string(),
            IsTopLevelCategory => "is_top_level_category".to_string(),
            BsrPercentage => "bsr_percentage".to_string(),
            Width => "width".to_string(),
            Height => "height".to_string(),
            Length => "length".to_string(),
            Weight => "weight".to_string(),
            CompetitiveSellers => "competitive_sellers".to_string(),
            FinancialsInboundShipping => "financials.inbound_shipping".to_string(),
            FinancialsPrepCost => "financials.prep_cost".to_string(),
            FinancialsFbaStorageFees => "financials.fba_storage_fees".to_string(),
            AmazonFeesPerItemFee => "amazon_fees.per_item_fee".to_string(),
            AmazonFeesFbaFees => "amazon_fees.fba_fees".to_string(),
            AmazonFeesVariableClosingFee => "amazon_fees.variable_closing_fee".to_string(),
            AmazonFeesReferralFee => "amazon_fees.referral_fee".to_string(),
            FinancialsNetRevenue => "financials.net_revenue".to_string(),
            FinancialsProfit => "financials.profit".to_string(),
            FinancialsMargin => "financials.margin".to_string(),
            FinancialsRoi => "financials.roi".to_string(),
            InputIdentifier => "inputs.identifier".to_string(),
            InputCost => "inputs.cost".to_string(),
            InputDiscountPerProduct => "inputs.per_product_discount".to_string(),
            InputDiscountCost => "inputs.discount_cost".to_string(),
            InputTotalCogs => "inputs.total_cogs".to_string(),
            InputStock => "inputs.stock".to_string(),
            InputMap => "inputs.map".to_string(),
            InputSupplierTitle => "inputs.supplier_title".to_string(),
            InputSupplierSku => "inputs.supplier_sku".to_string(),
            InputSupplierImage => "inputs.supplier_image".to_string(),
            InputSupplierPackQuantity => "inputs.supplier_pack_quantity".to_string(),
            SizeTier => "size_tier".to_string(),
            LowestPriceNewFba => "lowest_price_new_fba".to_string(),
            LowestPriceUsedFba => "lowest_price_used_fba".to_string(),
            LowestPriceNewFbm => "lowest_price_new_fbm".to_string(),
            LowestPriceUsedFbm => "lowest_price_used_fbm".to_string(),
            NewFbaOffersCount => "number_of_new_fba_offers".to_string(),
            NewFbmOffersCount => "number_of_new_fbm_offers".to_string(),
            IsBrandBlocklisted => "is_brand_blocklisted".to_string(),
            IsMeltable => "is_meltable".to_string(),
            AddToAmazonLink => "add_to_amazon_link".to_string(),
            InputCustom(i) => format!("inputs.custom.{}", i),
        }
    }
}

impl TryFrom<&str> for XLSXColumn {
    type Error = XError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "checkbox" => Ok(XLSXColumn::Checkbox),
            "notes" => Ok(XLSXColumn::Notes),
            "chart" => Ok(XLSXColumn::Chart),
            "flags" => Ok(XLSXColumn::Flags),
            "errors" => Ok(XLSXColumn::Errors),
            "asin" => Ok(XLSXColumn::Asin),
            "amazon_title" => Ok(XLSXColumn::AmazonTitle),
            "fba_eligibility" => Ok(XLSXColumn::FbaEligibility),
            "image" | "images" => Ok(XLSXColumn::Images),
            "size" | "size_name" => Ok(XLSXColumn::SizeName),
            "brand" => Ok(XLSXColumn::Brand),
            "color" => Ok(XLSXColumn::Color),
            "parent_asin" => Ok(XLSXColumn::ParentAsin),
            "variations_list" => Ok(XLSXColumn::VariationsList),
            "number_of_variations" => Ok(XLSXColumn::NumberOfVariations),
            "amazon_pack_quantity" => Ok(XLSXColumn::AmazonPackQuantity),
            "small_and_light_eligible" => Ok(XLSXColumn::SmallAndLightEligible),
            // "small_and_light_eligible_reasons" => Ok(XLSXColumn::SmallAndLightEligibleReasons),
            "buybox_price" => Ok(XLSXColumn::BuyboxPrice),
            "buybox_price_new" => Ok(XLSXColumn::BuyboxPriceNew),
            "buybox_price_used" => Ok(XLSXColumn::BuyboxPriceUsed),
            "number_of_total_offers" | "total_offers_count" => Ok(XLSXColumn::TotalOffersCount),
            "rank" => Ok(XLSXColumn::Rank),
            "category" => Ok(XLSXColumn::Category),
            "is_top_level_category" => Ok(XLSXColumn::IsTopLevelCategory),
            "bsr_percentage" => Ok(XLSXColumn::BsrPercentage),
            "width" => Ok(XLSXColumn::Width),
            "height" => Ok(XLSXColumn::Height),
            "length" => Ok(XLSXColumn::Length),
            "weight" => Ok(XLSXColumn::Weight),
            "number_of_competitive_sellers" | "competitive_sellers" => {
                Ok(XLSXColumn::CompetitiveSellers)
            }
            "financials_inbound_shipping" | "inbound_shipping" => {
                Ok(XLSXColumn::FinancialsInboundShipping)
            }
            "financials_prep_cost" | "prep_cost" => Ok(XLSXColumn::FinancialsPrepCost),
            "financials_fba_storage_fees" | "fba_storage_fees" => {
                Ok(XLSXColumn::FinancialsFbaStorageFees)
            }
            "amazon_fees_per_item_fee" | "per_item_fee" => Ok(XLSXColumn::AmazonFeesPerItemFee),
            "amazon_fees_fba_fees" | "fba_fees" => Ok(XLSXColumn::AmazonFeesFbaFees),
            "amazon_fees_variable_closing_fee" | "variable_closing_fee" => {
                Ok(XLSXColumn::AmazonFeesVariableClosingFee)
            }
            "amazon_fees_referral_fee" | "referral_fee" => Ok(XLSXColumn::AmazonFeesReferralFee),
            "financials_net_revenue" | "net_revenue" => Ok(XLSXColumn::FinancialsNetRevenue),
            "financials_profit" | "profit" => Ok(XLSXColumn::FinancialsProfit),
            "financials_margin" | "margin" => Ok(XLSXColumn::FinancialsMargin),
            "financials_roi" | "roi" => Ok(XLSXColumn::FinancialsRoi),
            "input_identifier" | "inputs.identifier" => Ok(XLSXColumn::InputIdentifier),
            "input_cost" | "inputs.cost" => Ok(XLSXColumn::InputCost),
            "input_discount_per_product" | "inputs.per_product_discount" => {
                Ok(XLSXColumn::InputDiscountPerProduct)
            }
            "input_discount_cost" | "inputs.discount_cost" => Ok(XLSXColumn::InputDiscountCost),
            "input_total_cogs" | "inputs.total_cogs" => Ok(XLSXColumn::InputTotalCogs),
            "input_stock" | "inputs.stock" => Ok(XLSXColumn::InputStock),
            "input_map" | "inputs.map" => Ok(XLSXColumn::InputMap),
            "input_supplier_title" | "inputs.supplier_title" => Ok(XLSXColumn::InputSupplierTitle),
            "input_supplier_sku" | "inputs.supplier_sku" => Ok(XLSXColumn::InputSupplierSku),
            "input_supplier_image" | "inputs.supplier_image" => Ok(XLSXColumn::InputSupplierImage),
            "input_supplier_pack_quantity" | "inputs.supplier_pack_quantity" => {
                Ok(XLSXColumn::InputSupplierPackQuantity)
            }
            "size_tier" => Ok(XLSXColumn::SizeTier),
            "lowest_price_new_fba" => Ok(XLSXColumn::LowestPriceNewFba),
            "lowest_price_used_fba" => Ok(XLSXColumn::LowestPriceUsedFba),
            "lowest_price_new_fbm" => Ok(XLSXColumn::LowestPriceNewFbm),
            "lowest_price_used_fbm" => Ok(XLSXColumn::LowestPriceUsedFbm),
            "new_fba_offers_count" | "number_of_new_fba_offers" => {
                Ok(XLSXColumn::NewFbaOffersCount)
            }
            "new_fbm_offers_count" | "number_of_new_fbm_offers" => {
                Ok(XLSXColumn::NewFbmOffersCount)
            }
            "is_brand_blocklisted" => Ok(XLSXColumn::IsBrandBlocklisted),
            "is_meltable" => Ok(XLSXColumn::IsMeltable),
            "add_to_amazon_link" => Ok(XLSXColumn::AddToAmazonLink),
            s => {
                if s.starts_with("inputs.custom.") {
                    let i = s.replace("inputs.custom.", "").parse::<u32>()?;
                    Ok(XLSXColumn::InputCustom(i))
                } else {
                    Err(XError::Other(format!("invalid XLSX column: {}", s)))
                }
            }
        }
    }
}

impl XLSXColumn {
    pub fn display<'a>(&'a self, m: Marketplace, item: &'a Product) -> Display {
        use crate::xlsx::display::Display::Empty;

        let str_list_opt =
            |v: &Option<Vec<String>>| v.as_ref().map(|v| v.join(", ").to_string()).into();

        match self {
            XLSXColumn::Checkbox => Empty,
            XLSXColumn::Notes => item.note.as_ref().into(),
            XLSXColumn::Chart => Empty, // @todo?
            XLSXColumn::Flags => str_list_opt(&item.flags),
            XLSXColumn::Errors => str_list_opt(&item.errors),
            XLSXColumn::Asin => {
                if item.has_errors() {
                    return Empty;
                }

                Display::Link {
                    url: format!("https://www.{}/dp/{}", m.uri(), item.asin.as_ref().unwrap()),
                    text: item.asin.as_ref().unwrap().to_string(),
                }
            }
            XLSXColumn::AmazonTitle => item.amazon_title.as_ref().into(),
            XLSXColumn::FbaEligibility => {
                if item.has_errors() {
                    Empty
                } else if item.inbound_eligibility.is_none() {
                    "Unknown".into()
                } else {
                    let v = item.inbound_eligibility.as_ref().unwrap();
                    v.eligible.into()
                }
            }
            XLSXColumn::Images => {
                let img = match &item.images {
                    Some(v) => v.get_main_image(m),
                    None => return Empty,
                };

                match img {
                    Some(img) => Display::Link {
                        url: img,
                        text: "Image".to_string(),
                    },
                    None => Empty,
                }
            }
            XLSXColumn::SizeName => item.size_name.as_ref().into(),
            XLSXColumn::Brand => item.brand.as_ref().into(),
            XLSXColumn::Color => item.color.as_ref().into(),
            XLSXColumn::ParentAsin => item.parent_asin.as_ref().into(),
            XLSXColumn::VariationsList => str_list_opt(&item.variations_list),
            XLSXColumn::NumberOfVariations => item.number_of_variations.unwrap_or(0).into(),
            XLSXColumn::AmazonPackQuantity => item.amazon_pack_quantity.into(),
            XLSXColumn::SmallAndLightEligible => item.small_and_light_eligible.into(),
            XLSXColumn::SmallAndLightEligibleReasons => {
                item.small_and_light_eligible_reasons.clone().into()
            }
            XLSXColumn::BuyboxPrice => OptMoney(item.buybox_price),
            XLSXColumn::BuyboxPriceNew => OptMoney(item.buybox_price_new),
            XLSXColumn::BuyboxPriceUsed => OptMoney(item.buybox_price_used),
            XLSXColumn::TotalOffersCount => item.total_offers_count.into(),
            XLSXColumn::Rank => item.rank.into(),
            XLSXColumn::Category => item.category.as_ref().into(),
            XLSXColumn::IsTopLevelCategory => item.is_top_level_category.into(),
            XLSXColumn::BsrPercentage => item.bsr_percentage.map(|x| x as f64 / 10000.0).into(),
            XLSXColumn::Width => {
                if item.has_errors() {
                    Empty
                } else {
                    let v = item.dimensions.get_width();
                    match v {
                        Some(v) if v == 0.0 => return Empty,
                        _ => {}
                    };
                    v.into()
                }
            }
            XLSXColumn::Height => {
                if item.has_errors() {
                    Empty
                } else {
                    let v = item.dimensions.get_height();
                    match v {
                        Some(v) if v == 0.0 => return Empty,
                        _ => {}
                    };
                    v.into()
                }
            }
            XLSXColumn::Length => {
                if item.has_errors() {
                    Empty
                } else {
                    let v = item.dimensions.get_length();
                    match v {
                        Some(v) if v == 0.0 => return Empty,
                        _ => {}
                    };
                    v.into()
                }
            }
            XLSXColumn::Weight => {
                if item.has_errors() {
                    Empty
                } else {
                    let v = item.dimensions.get_weight();
                    match v {
                        Some(v) if v == 0.0 => return Empty,
                        _ => {}
                    };
                    v.into()
                }
            }
            XLSXColumn::CompetitiveSellers => item.competitive_sellers.into(),
            XLSXColumn::FinancialsInboundShipping => OptMoney(item.financials.inbound_shipping),
            XLSXColumn::FinancialsPrepCost => OptMoney(item.financials.prep_cost),
            XLSXColumn::FinancialsFbaStorageFees => OptMoney(item.financials.fba_storage_fees),
            XLSXColumn::AmazonFeesPerItemFee => OptMoney(item.amazon_fees.per_item_fee),
            XLSXColumn::AmazonFeesFbaFees => OptMoney(item.amazon_fees.fba_fees),
            XLSXColumn::AmazonFeesVariableClosingFee => {
                OptMoney(item.amazon_fees.variable_closing_fee)
            }
            XLSXColumn::AmazonFeesReferralFee => OptMoney(item.amazon_fees.referral_fee),
            XLSXColumn::FinancialsNetRevenue => OptMoney(item.financials.net_revenue),
            XLSXColumn::FinancialsProfit => OptMoney(item.financials.profit),
            XLSXColumn::FinancialsMargin => item
                .financials
                .margin
                .as_ref()
                .map(|x| *x as f64 / 10000.0)
                .into(),
            XLSXColumn::FinancialsRoi => item
                .financials
                .roi
                .as_ref()
                .map(|x| *x as f64 / 10000.0)
                .into(),
            XLSXColumn::InputIdentifier => item.inputs.identifier.as_ref().into(),
            XLSXColumn::InputCost => OptMoney(item.inputs.cost),
            XLSXColumn::InputDiscountPerProduct => item
                .inputs
                .discount_per_product
                .as_ref()
                .map(|x| *x as f64 / 10000.0)
                .into(),
            XLSXColumn::InputDiscountCost => OptMoney(item.inputs.discount_cost),
            XLSXColumn::InputTotalCogs => OptMoney(item.inputs.total_cogs),
            XLSXColumn::InputStock => item.inputs.stock.into(),
            XLSXColumn::InputMap => OptMoney(item.inputs.map),
            XLSXColumn::InputSupplierTitle => item.inputs.supplier_title.as_ref().into(),
            XLSXColumn::InputSupplierSku => item.inputs.supplier_sku.as_ref().into(),
            XLSXColumn::InputSupplierImage => {
                let img = match &item.inputs.supplier_image {
                    Some(v) => v,
                    None => return Empty,
                };

                Display::Link {
                    url: img.to_owned(),
                    text: "Image".to_string(),
                }
            }
            XLSXColumn::InputSupplierPackQuantity => item.inputs.supplier_pack_quantity.into(),
            XLSXColumn::SizeTier => item.size_tier.as_ref().into(),
            XLSXColumn::LowestPriceNewFba => OptMoney(item.lowest_price_new_fba),
            XLSXColumn::LowestPriceUsedFba => OptMoney(item.lowest_price_used_fba),
            XLSXColumn::LowestPriceNewFbm => OptMoney(item.lowest_price_new_fbm),
            XLSXColumn::LowestPriceUsedFbm => OptMoney(item.lowest_price_used_fbm),
            XLSXColumn::NewFbaOffersCount => item.new_fba_offers_count.into(),
            XLSXColumn::NewFbmOffersCount => item.new_fbm_offers_count.into(),
            XLSXColumn::IsBrandBlocklisted => item.is_brand_blocklisted.into(),
            XLSXColumn::IsMeltable => item.is_meltable.into(),
            XLSXColumn::AddToAmazonLink => {
                let asin = match &item.asin {
                    None => return Empty,
                    Some(v) if v.is_empty() => return Empty,
                    Some(v) => v,
                };

                let url = format!("{}product-search/search?q={}", m.seller_central_url(), asin);

                Display::Link {
                    url,
                    text: "Create Listing".to_string(),
                }
            }
            XLSXColumn::InputCustom(i) => item
                .inputs
                .custom_columns
                .as_ref()
                .map(|x| &x[*i as usize])
                .unwrap()
                .into(),
        }
    }

    pub fn visible_on_products_sheet(&self) -> bool {
        match self {
            XLSXColumn::Checkbox => false,
            XLSXColumn::Notes => true,
            XLSXColumn::Chart => false,
            XLSXColumn::Flags => false,
            XLSXColumn::Errors => false,
            XLSXColumn::Asin => true,
            XLSXColumn::AmazonTitle => true,
            XLSXColumn::FbaEligibility => true,
            XLSXColumn::Images => true,
            XLSXColumn::SizeName => true,
            XLSXColumn::Brand => true,
            XLSXColumn::Color => true,
            XLSXColumn::ParentAsin => true,
            XLSXColumn::VariationsList => true,
            XLSXColumn::NumberOfVariations => true,
            XLSXColumn::AmazonPackQuantity => true,
            XLSXColumn::SmallAndLightEligible => true,
            XLSXColumn::SmallAndLightEligibleReasons => true,
            XLSXColumn::BuyboxPrice => true,
            XLSXColumn::BuyboxPriceNew => true,
            XLSXColumn::BuyboxPriceUsed => true,
            XLSXColumn::TotalOffersCount => true,
            XLSXColumn::Rank => true,
            XLSXColumn::Category => true,
            XLSXColumn::IsTopLevelCategory => true,
            XLSXColumn::BsrPercentage => true,
            XLSXColumn::Width => true,
            XLSXColumn::Height => true,
            XLSXColumn::Length => true,
            XLSXColumn::Weight => true,
            XLSXColumn::CompetitiveSellers => true,
            XLSXColumn::FinancialsInboundShipping => true,
            XLSXColumn::FinancialsPrepCost => true,
            XLSXColumn::FinancialsFbaStorageFees => true,
            XLSXColumn::AmazonFeesPerItemFee => true,
            XLSXColumn::AmazonFeesFbaFees => true,
            XLSXColumn::AmazonFeesVariableClosingFee => true,
            XLSXColumn::AmazonFeesReferralFee => true,
            XLSXColumn::FinancialsNetRevenue => true,
            XLSXColumn::FinancialsProfit => true,
            XLSXColumn::FinancialsMargin => true,
            XLSXColumn::FinancialsRoi => true,
            XLSXColumn::InputIdentifier => true,
            XLSXColumn::InputCost => true,
            XLSXColumn::InputDiscountPerProduct => true,
            XLSXColumn::InputDiscountCost => true,
            XLSXColumn::InputTotalCogs => true,
            XLSXColumn::InputStock => true,
            XLSXColumn::InputMap => true,
            XLSXColumn::InputSupplierTitle => true,
            XLSXColumn::InputSupplierSku => true,
            XLSXColumn::InputSupplierImage => true,
            XLSXColumn::InputSupplierPackQuantity => true,
            XLSXColumn::SizeTier => true,
            XLSXColumn::LowestPriceNewFba => true,
            XLSXColumn::LowestPriceUsedFba => true,
            XLSXColumn::LowestPriceNewFbm => true,
            XLSXColumn::LowestPriceUsedFbm => true,
            XLSXColumn::NewFbaOffersCount => true,
            XLSXColumn::NewFbmOffersCount => true,
            XLSXColumn::IsBrandBlocklisted => true,
            XLSXColumn::IsMeltable => true,
            XLSXColumn::AddToAmazonLink => true,
            XLSXColumn::InputCustom(_) => true,
        }
    }

    pub fn visible_on_errors_sheet(&self) -> bool {
        match self {
            XLSXColumn::Checkbox => false,
            XLSXColumn::Notes => true,
            XLSXColumn::Chart => false,
            XLSXColumn::Flags => false,
            XLSXColumn::Errors => true,
            XLSXColumn::Asin => false,
            XLSXColumn::AmazonTitle => false,
            XLSXColumn::FbaEligibility => false,
            XLSXColumn::Images => false,
            XLSXColumn::SizeName => false,
            XLSXColumn::Brand => false,
            XLSXColumn::Color => false,
            XLSXColumn::ParentAsin => false,
            XLSXColumn::VariationsList => false,
            XLSXColumn::NumberOfVariations => false,
            XLSXColumn::AmazonPackQuantity => false,
            XLSXColumn::SmallAndLightEligible => false,
            XLSXColumn::SmallAndLightEligibleReasons => false,
            XLSXColumn::BuyboxPrice => false,
            XLSXColumn::BuyboxPriceNew => false,
            XLSXColumn::BuyboxPriceUsed => false,
            XLSXColumn::TotalOffersCount => false,
            XLSXColumn::Rank => false,
            XLSXColumn::Category => false,
            XLSXColumn::IsTopLevelCategory => false,
            XLSXColumn::BsrPercentage => false,
            XLSXColumn::Width => false,
            XLSXColumn::Height => false,
            XLSXColumn::Length => false,
            XLSXColumn::Weight => false,
            XLSXColumn::CompetitiveSellers => false,
            XLSXColumn::FinancialsInboundShipping => false,
            XLSXColumn::FinancialsPrepCost => false,
            XLSXColumn::FinancialsFbaStorageFees => false,
            XLSXColumn::AmazonFeesPerItemFee => false,
            XLSXColumn::AmazonFeesFbaFees => false,
            XLSXColumn::AmazonFeesVariableClosingFee => false,
            XLSXColumn::AmazonFeesReferralFee => false,
            XLSXColumn::FinancialsNetRevenue => false,
            XLSXColumn::FinancialsProfit => false,
            XLSXColumn::FinancialsMargin => false,
            XLSXColumn::FinancialsRoi => false,
            XLSXColumn::InputIdentifier => true,
            XLSXColumn::InputCost => true,
            XLSXColumn::InputDiscountPerProduct => true,
            XLSXColumn::InputDiscountCost => true,
            XLSXColumn::InputTotalCogs => true,
            XLSXColumn::InputStock => true,
            XLSXColumn::InputMap => true,
            XLSXColumn::InputSupplierTitle => true,
            XLSXColumn::InputSupplierSku => true,
            XLSXColumn::InputSupplierImage => true,
            XLSXColumn::InputSupplierPackQuantity => true,
            XLSXColumn::SizeTier => false,
            XLSXColumn::LowestPriceNewFba => false,
            XLSXColumn::LowestPriceUsedFba => false,
            XLSXColumn::LowestPriceNewFbm => false,
            XLSXColumn::LowestPriceUsedFbm => false,
            XLSXColumn::NewFbaOffersCount => false,
            XLSXColumn::NewFbmOffersCount => false,
            XLSXColumn::IsBrandBlocklisted => false,
            XLSXColumn::IsMeltable => false,
            XLSXColumn::AddToAmazonLink => false,
            XLSXColumn::InputCustom(_) => true,
        }
    }

    pub fn header<'a>(&self, scan: &'a Scan) -> &'a str {
        match self {
            XLSXColumn::Checkbox => "Checkbox",
            XLSXColumn::Notes => "Notes",
            XLSXColumn::Chart => "Chart",
            XLSXColumn::Flags => "Flags",
            XLSXColumn::Errors => "Errors",
            XLSXColumn::Asin => "ASIN",
            XLSXColumn::AmazonTitle => "Amazon Title",
            XLSXColumn::FbaEligibility => "FBA Eligible",
            XLSXColumn::Images => "Image",
            XLSXColumn::SizeName => "Size",
            XLSXColumn::Brand => "Brand",
            XLSXColumn::Color => "Color",
            XLSXColumn::ParentAsin => "Parent ASIN",
            XLSXColumn::VariationsList => "Variations List",
            XLSXColumn::NumberOfVariations => "# Variations",
            XLSXColumn::AmazonPackQuantity => "Amazon Pack Quantity",
            XLSXColumn::SmallAndLightEligible => "Small & Light Eligible",
            XLSXColumn::SmallAndLightEligibleReasons => "Small & Light Ineligible Reasons",
            XLSXColumn::BuyboxPrice => "Buybox Price",
            XLSXColumn::BuyboxPriceNew => "Buybox Price New",
            XLSXColumn::BuyboxPriceUsed => "Buybox Price Used",
            XLSXColumn::TotalOffersCount => "# Total Offers",
            XLSXColumn::Rank => "Rank",
            XLSXColumn::Category => "Category",
            XLSXColumn::IsTopLevelCategory => "Top Level Category",
            XLSXColumn::BsrPercentage => "BSR Percentage",
            XLSXColumn::Width => "Width",
            XLSXColumn::Height => "Height",
            XLSXColumn::Length => "Length",
            XLSXColumn::Weight => "Weight",
            XLSXColumn::CompetitiveSellers => "Competitive Sellers",
            XLSXColumn::FinancialsInboundShipping => "Inbound Shipping",
            XLSXColumn::FinancialsPrepCost => "Prep Cost",
            XLSXColumn::FinancialsFbaStorageFees => "FBA Storage Fees",
            XLSXColumn::AmazonFeesPerItemFee => "Per Item Fee",
            XLSXColumn::AmazonFeesFbaFees => "FBA Fees",
            XLSXColumn::AmazonFeesVariableClosingFee => "Variable Closing Fee",
            XLSXColumn::AmazonFeesReferralFee => "Referral Fee",
            XLSXColumn::FinancialsNetRevenue => "Sale Proceeds",
            XLSXColumn::FinancialsProfit => "Profit",
            XLSXColumn::FinancialsMargin => "Margin",
            XLSXColumn::FinancialsRoi => "ROI",
            XLSXColumn::InputIdentifier => "Identifier",
            XLSXColumn::InputCost => "Cost",
            XLSXColumn::InputDiscountPerProduct => "Product Discount",
            XLSXColumn::InputDiscountCost => "Adjusted Cost",
            XLSXColumn::InputTotalCogs => "Total COGS",
            XLSXColumn::InputStock => "Stock",
            XLSXColumn::InputMap => "MAP",
            XLSXColumn::InputSupplierTitle => "Supplier Title",
            XLSXColumn::InputSupplierSku => "Supplier SKU",
            XLSXColumn::InputSupplierImage => "Supplier Image",
            XLSXColumn::InputSupplierPackQuantity => "Supplier Pack Quantity",
            XLSXColumn::SizeTier => "Size Tier",
            XLSXColumn::LowestPriceNewFba => "Lowest Price New FBA",
            XLSXColumn::LowestPriceUsedFba => "Lowest Price Used FBA",
            XLSXColumn::LowestPriceNewFbm => "Lowest Price New FBM",
            XLSXColumn::LowestPriceUsedFbm => "Lowest Price Used FBM",
            XLSXColumn::NewFbaOffersCount => "# New FBA Offers",
            XLSXColumn::NewFbmOffersCount => "# New FBM Offers",
            XLSXColumn::IsBrandBlocklisted => "Blocklisted Brand",
            XLSXColumn::IsMeltable => "Meltable",
            XLSXColumn::AddToAmazonLink => "Create Listing",
            XLSXColumn::InputCustom(i) => scan.custom_col_header(*i).unwrap_or(""),
        }
    }

    pub fn formula(&self) -> Option<&str> {
        match self {
            XLSXColumn::FinancialsInboundShipping => Some(INBOUND_SHIPPING),
            XLSXColumn::FinancialsPrepCost => Some(PREP_COST),
            XLSXColumn::FinancialsNetRevenue => Some(NET_REVENUE),
            XLSXColumn::FinancialsProfit => Some(PROFIT),
            XLSXColumn::FinancialsMargin => Some(MARGIN),
            XLSXColumn::FinancialsRoi => Some(ROI),
            XLSXColumn::InputDiscountCost => Some(DISCOUNT_COST),
            XLSXColumn::InputTotalCogs => Some(TOTAL_COGS),
            _ => None,
        }
    }

    // @todo check that these are all right
    pub fn sql_column(&self) -> String {
        use XLSXColumn::*;

        match self {
            Checkbox => panic!("Checkbox is not a SQL column"),
            Notes => panic!("Notes is not a SQL column"),
            Chart => panic!("Chart is not a SQL column"),
            Flags => "flags".to_string(),
            Errors => "errors".to_string(),
            Asin => "asin".to_string(),
            AmazonTitle => "amazon_title".to_string(),
            FbaEligibility => "fba_eligibility".to_string(), // @todo?
            Images => "images".to_string(),
            SizeName => "size_name".to_string(),
            Brand => "brand".to_string(),
            Color => "color".to_string(),
            ParentAsin => "parent_asin".to_string(),
            VariationsList => "variations_list".to_string(),
            NumberOfVariations => "number_of_variations".to_string(),
            AmazonPackQuantity => "amazon_pack_quantity".to_string(),
            SmallAndLightEligible => "small_and_light_eligible".to_string(),
            SmallAndLightEligibleReasons => "small_and_light_eligible_reasons".to_string(),
            BuyboxPrice => "buybox_price".to_string(),
            BuyboxPriceNew => "buybox_price_new".to_string(),
            BuyboxPriceUsed => "buybox_price_used".to_string(),
            TotalOffersCount => "total_offers_count".to_string(),
            Rank => "rank".to_string(),
            Category => "category".to_string(),
            IsTopLevelCategory => "is_top_level_category".to_string(),
            BsrPercentage => "bsr_percentage".to_string(),
            Width => "width".to_string(),
            Height => "height".to_string(),
            Length => "length".to_string(),
            Weight => "weight".to_string(),
            CompetitiveSellers => "competitive_sellers".to_string(),
            FinancialsInboundShipping => "financials_inbound_shipping".to_string(),
            FinancialsPrepCost => "financials_prep_cost".to_string(),
            FinancialsFbaStorageFees => "financials_fba_storage_fees".to_string(),
            AmazonFeesPerItemFee => "amazon_fees_per_item_fee".to_string(),
            AmazonFeesFbaFees => "amazon_fees_fba_fees".to_string(),
            AmazonFeesVariableClosingFee => "amazon_fees_variable_closing_fee".to_string(),
            AmazonFeesReferralFee => "amazon_fees_referral_fee".to_string(),
            FinancialsNetRevenue => "financials_net_revenue".to_string(),
            FinancialsProfit => "financials_profit".to_string(),
            FinancialsMargin => "financials_margin".to_string(),
            FinancialsRoi => "financials_roi".to_string(),
            InputIdentifier => "input_identifier".to_string(),
            InputCost => "input_cost".to_string(),
            InputDiscountPerProduct => "input_discount_per_product".to_string(),
            InputDiscountCost => "input_discount_cost".to_string(),
            InputTotalCogs => "input_total_cogs".to_string(),
            InputStock => "input_stock".to_string(),
            InputMap => "input_map".to_string(),
            InputSupplierTitle => "input_supplier_title".to_string(),
            InputSupplierSku => "input_supplier_sku".to_string(),
            InputSupplierImage => "input_supplier_image".to_string(),
            InputSupplierPackQuantity => "input_supplier_pack_quantity".to_string(),
            SizeTier => "size_tier".to_string(),
            LowestPriceNewFba => "lowest_price_new_fba".to_string(),
            LowestPriceUsedFba => "lowest_price_used_fba".to_string(),
            LowestPriceNewFbm => "lowest_price_new_fbm".to_string(),
            LowestPriceUsedFbm => "lowest_price_used_fbm".to_string(),
            NewFbaOffersCount => "new_fba_offers_count".to_string(),
            NewFbmOffersCount => "new_fbm_offers_count".to_string(),
            IsBrandBlocklisted => "is_brand_blocklisted".to_string(),
            IsMeltable => "is_meltable".to_string(),
            AddToAmazonLink => panic!("AddToAmazonLink is not a SQL column"),
            // This is a postgres jsonb array access for the nth item
            InputCustom(n) => format!("input_custom[{}]", n),
        }
    }
}
