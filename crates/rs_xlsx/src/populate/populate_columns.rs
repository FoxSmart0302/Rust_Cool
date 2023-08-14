use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum SqlColumn {
    Id,
    Flags,
    Errors,
    InputIdentifier,
    InputCost,
    InputStock,
    InputMap,
    InputSupplierTitle,
    InputSupplierSku,
    InputSupplierImage,
    InputSupplierPackQuantity,
    InputDiscountPerProduct,
    InputDiscountSupplier,
    InputDiscountCost,
    InputTotalCogs,
    InputCustom,
    Asin,
    Offers,
    OffersTotal,
    OffersListPrice,
    //-- @todo buybox eligible offers,
    //-- @todo number of offers,
    //-- @todo lowest prices,
    //-- @todo buybox prices,
    // /*
    Images,
    AmazonTitle,
    IsTopLevelCategory,
    CategoryRaw,
    Category,
    Rank,
    BuyboxPrice,
    AmazonPackQuantity,
    NumberOfVariations,
    VariationsList,
    ParentAsin,
    //SalesRankCategories,
    //SalesRankRanks,
    SalesRanks,
    PackageLength,
    PackageWidth,
    PackageHeight,
    PackageWeight,
    PackageLengthUnit,
    PackageWidthUnit,
    PackageHeightUnit,
    PackageWeightUnit,
    ItemLength,
    ItemWidth,
    ItemHeight,
    ItemWeight,
    ItemLengthUnit,
    ItemWidthUnit,
    ItemHeightUnit,
    ItemWeightUnit,
    AmazonFeesPerItemFee,
    AmazonFeesFbaFees,
    AmazonFeesVariableClosingFee,
    AmazonFeesReferralFee,
    AmazonFeesError,
    CompetitiveSellers,
    Brand,
    Color,
    SizeName,
    ListingRestrictions,
    FinancialsInboundShipping,
    FinancialsPrepCost,
    FinancialsFbaStorageFees,
    FinancialsNetRevenue,
    FinancialsProfit,
    FinancialsSaleProceeds,
    FinancialsMargin,
    FinancialsRoi,
    SizeTier,
    LowestPriceNewFba,
    LowestPriceUsedFba,
    LowestPriceNewFbm,
    LowestPriceUsedFbm,
    BuyboxPriceNew,
    BuyboxPriceUsed,
    TotalOffersCount,
    IsBrandBlocklisted,
    NewFbaOffersCount,
    NewFbmOffersCount,
    IsAdult,
    IsHazmat,
    IsMeltable,
    SmallAndLightEligible,
    SmallAndLightEligibleReasons,
    BsrPercentage,
}

impl SqlColumn {
    pub fn as_str(&self) -> &'static str {
        match self {
            SqlColumn::Id => "id",
            SqlColumn::Flags => "flags",
            SqlColumn::Errors => "errors",
            SqlColumn::InputIdentifier => "input_identifier",
            SqlColumn::InputCost => "input_cost",
            SqlColumn::InputStock => "input_stock",
            SqlColumn::InputMap => "input_map",
            SqlColumn::InputSupplierTitle => "input_supplier_title",
            SqlColumn::InputSupplierSku => "input_supplier_sku",
            SqlColumn::InputSupplierImage => "input_supplier_image",
            SqlColumn::InputSupplierPackQuantity => "input_supplier_pack_quantity",
            SqlColumn::InputDiscountPerProduct => "input_discount_per_product",
            SqlColumn::InputDiscountSupplier => "input_discount_supplier",
            SqlColumn::InputDiscountCost => "input_discount_cost",
            SqlColumn::InputTotalCogs => "input_total_cogs",
            SqlColumn::InputCustom => "input_custom",
            SqlColumn::Asin => "asin",
            SqlColumn::Offers => "offers",
            SqlColumn::OffersTotal => "offers_total",
            SqlColumn::OffersListPrice => "offers_list_price",
            SqlColumn::Images => "images",
            SqlColumn::AmazonTitle => "amazon_title",
            SqlColumn::IsTopLevelCategory => "is_top_level_category",
            SqlColumn::CategoryRaw => "category_raw",
            SqlColumn::Category => "category",
            SqlColumn::Rank => "rank",
            SqlColumn::BuyboxPrice => "buybox_price",
            SqlColumn::AmazonPackQuantity => "amazon_pack_quantity",
            SqlColumn::NumberOfVariations => "number_of_variations",
            SqlColumn::VariationsList => "variations_list",
            SqlColumn::ParentAsin => "parent_asin",
            SqlColumn::SalesRanks => "sales_ranks",
            SqlColumn::PackageLength => "package_length",
            SqlColumn::PackageWidth => "package_width",
            SqlColumn::PackageHeight => "package_height",
            SqlColumn::PackageWeight => "package_weight",
            SqlColumn::PackageLengthUnit => "package_length_unit",
            SqlColumn::PackageWidthUnit => "package_width_unit",
            SqlColumn::PackageHeightUnit => "package_height_unit",
            SqlColumn::PackageWeightUnit => "package_weight_unit",
            SqlColumn::ItemLength => "item_length",
            SqlColumn::ItemWidth => "item_width",
            SqlColumn::ItemHeight => "item_height",
            SqlColumn::ItemWeight => "item_weight",
            SqlColumn::ItemLengthUnit => "item_length_unit",
            SqlColumn::ItemWidthUnit => "item_width_unit",
            SqlColumn::ItemHeightUnit => "item_height_unit",
            SqlColumn::ItemWeightUnit => "item_weight_unit",
            SqlColumn::AmazonFeesPerItemFee => "amazon_fees_per_item_fee",
            SqlColumn::AmazonFeesFbaFees => "amazon_fees_fba_fees",
            SqlColumn::AmazonFeesVariableClosingFee => "amazon_fees_variable_closing_fee",
            SqlColumn::AmazonFeesReferralFee => "amazon_fees_referral_fee",
            SqlColumn::AmazonFeesError => "amazon_fees_error",
            SqlColumn::CompetitiveSellers => "competitive_sellers",
            SqlColumn::Brand => "brand",
            SqlColumn::Color => "color",
            SqlColumn::SizeName => "size_name",
            SqlColumn::ListingRestrictions => "listing_restrictions",
            SqlColumn::FinancialsInboundShipping => "financials_inbound_shipping",
            SqlColumn::FinancialsPrepCost => "financials_prep_cost",
            SqlColumn::FinancialsFbaStorageFees => "financials_fba_storage_fees",
            SqlColumn::FinancialsNetRevenue => "financials_net_revenue",
            SqlColumn::FinancialsProfit => "financials_profit",
            SqlColumn::FinancialsSaleProceeds => "financials_sale_proceeds",
            SqlColumn::FinancialsMargin => "financials_margin",
            SqlColumn::FinancialsRoi => "financials_roi",
            SqlColumn::SizeTier => "size_tier",
            SqlColumn::LowestPriceNewFba => "lowest_price_new_fba",
            SqlColumn::LowestPriceUsedFba => "lowest_price_used_fba",
            SqlColumn::LowestPriceNewFbm => "lowest_price_new_fbm",
            SqlColumn::LowestPriceUsedFbm => "lowest_price_used_fbm",
            SqlColumn::BuyboxPriceNew => "buybox_price_new",
            SqlColumn::BuyboxPriceUsed => "buybox_price_used",
            SqlColumn::TotalOffersCount => "total_offers_count",
            SqlColumn::IsBrandBlocklisted => "is_brand_blocklisted",
            SqlColumn::NewFbaOffersCount => "new_fba_offers_count",
            SqlColumn::NewFbmOffersCount => "new_fbm_offers_count",
            SqlColumn::IsAdult => "is_adult",
            SqlColumn::IsHazmat => "is_hazmat",
            SqlColumn::IsMeltable => "is_meltable",
            SqlColumn::SmallAndLightEligible => "small_and_light_eligible",
            SqlColumn::SmallAndLightEligibleReasons => "small_and_light_eligible_reasons",
            SqlColumn::BsrPercentage => "bsr_percentage",
        }
    }
    pub fn kind(&self) -> ColType {
        use ColType::*;

        match self {
            SqlColumn::Id => BigInt,
            SqlColumn::Flags => JsonB,
            SqlColumn::Errors => JsonB,
            SqlColumn::InputIdentifier => Text,
            SqlColumn::InputCost => Int,
            SqlColumn::InputStock => Int,
            SqlColumn::InputMap => Int,
            SqlColumn::InputSupplierTitle => Text,
            SqlColumn::InputSupplierSku => Text,
            SqlColumn::InputSupplierImage => Text, // 10
            SqlColumn::InputSupplierPackQuantity => Int,
            SqlColumn::InputDiscountPerProduct => Int,
            SqlColumn::InputDiscountSupplier => Int,
            SqlColumn::InputDiscountCost => Int,
            SqlColumn::InputTotalCogs => Int, // 15
            SqlColumn::InputCustom => JsonB,
            SqlColumn::Asin => VarChar(255),
            SqlColumn::Offers => JsonB,
            SqlColumn::OffersTotal => Int,
            SqlColumn::OffersListPrice => Int,
            SqlColumn::Images => JsonB,
            SqlColumn::AmazonTitle => Text,
            SqlColumn::IsTopLevelCategory => Boolean,
            SqlColumn::CategoryRaw => Text,
            SqlColumn::Category => Text,
            SqlColumn::Rank => Int,
            SqlColumn::BuyboxPrice => Int,
            SqlColumn::AmazonPackQuantity => Int,
            SqlColumn::NumberOfVariations => Int,
            SqlColumn::VariationsList => JsonB,
            SqlColumn::ParentAsin => VarChar(255),
            SqlColumn::SalesRanks => JsonB,
            SqlColumn::PackageLength => Float8,
            SqlColumn::PackageWidth => Float8,
            SqlColumn::PackageHeight => Float8,
            SqlColumn::PackageWeight => Float8,
            SqlColumn::PackageLengthUnit => VarChar(255),
            SqlColumn::PackageWidthUnit => VarChar(255),
            SqlColumn::PackageHeightUnit => VarChar(255),
            SqlColumn::PackageWeightUnit => VarChar(255),
            SqlColumn::ItemLength => Float8,
            SqlColumn::ItemWidth => Float8,
            SqlColumn::ItemHeight => Float8,
            SqlColumn::ItemWeight => Float8,
            SqlColumn::ItemLengthUnit => VarChar(255),
            SqlColumn::ItemWidthUnit => VarChar(255),
            SqlColumn::ItemHeightUnit => VarChar(255),
            SqlColumn::ItemWeightUnit => VarChar(255),
            SqlColumn::AmazonFeesPerItemFee => Int,
            SqlColumn::AmazonFeesFbaFees => Int,
            SqlColumn::AmazonFeesVariableClosingFee => Int,
            SqlColumn::AmazonFeesReferralFee => Int,
            SqlColumn::AmazonFeesError => VarChar(255),
            SqlColumn::CompetitiveSellers => Int,
            SqlColumn::Brand => VarChar(255),
            SqlColumn::Color => Text,
            SqlColumn::SizeName => VarChar(255),
            SqlColumn::ListingRestrictions => JsonB,
            SqlColumn::FinancialsInboundShipping => Int,
            SqlColumn::FinancialsPrepCost => Int,
            SqlColumn::FinancialsFbaStorageFees => Int,
            SqlColumn::FinancialsNetRevenue => Int,
            SqlColumn::FinancialsProfit => Int,
            SqlColumn::FinancialsSaleProceeds => Int,
            SqlColumn::FinancialsMargin => Int,
            SqlColumn::FinancialsRoi => Int,
            SqlColumn::SizeTier => VarChar(255),
            SqlColumn::LowestPriceNewFba => Int,
            SqlColumn::LowestPriceUsedFba => Int,
            SqlColumn::LowestPriceNewFbm => Int,
            SqlColumn::LowestPriceUsedFbm => Int,
            SqlColumn::BuyboxPriceNew => Int,
            SqlColumn::BuyboxPriceUsed => Int,
            SqlColumn::TotalOffersCount => Int,
            SqlColumn::IsBrandBlocklisted => Boolean,
            SqlColumn::NewFbaOffersCount => Int,
            SqlColumn::NewFbmOffersCount => Int,
            SqlColumn::IsAdult => Boolean,
            SqlColumn::IsHazmat => Boolean,
            SqlColumn::IsMeltable => Boolean,
            SqlColumn::SmallAndLightEligible => Boolean,
            SqlColumn::SmallAndLightEligibleReasons => SmallInt,
            SqlColumn::BsrPercentage => Int,
        }
    }
}

pub enum ColType {
    BigInt,
    Int,
    SmallInt,
    Float8,
    JsonB,
    Text,
    Boolean,
    VarChar(usize),
}

impl ToString for ColType {
    fn to_string(&self) -> String {
        match self {
            ColType::BigInt => "bigint".to_string(),
            ColType::Int => "int".to_string(),
            ColType::SmallInt => "smallint".to_string(),
            ColType::Float8 => "float8".to_string(),
            ColType::JsonB => "jsonb".to_string(),
            ColType::Text => "text".to_string(),
            ColType::Boolean => "boolean".to_string(),
            ColType::VarChar(n) => format!("varchar({n})"),
        }
    }
}

pub fn get_columns_list_as_string() -> String {
    let mut out = String::new();

    SqlColumn::iter().for_each(|x| {
        out.push_str(x.as_str());
        out.push(' ');
        out.push_str(&x.kind().to_string());
        out.push(',');
    });

    out.trim_end_matches(',').to_string()
}

pub fn get_columns_list() -> Vec<(&'static str, ColType)> {
    SqlColumn::iter().map(|x| (x.as_str(), x.kind())).collect()
}
