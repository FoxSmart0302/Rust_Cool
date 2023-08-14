use crate::error::{XError, XResult};
use crate::result_request::FilterValue;
use crate::xlsx::columns::XLSXColumn;
use composable_query_builder::ComposableQueryBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Filter {
    pub name: XLSXColumn,
    pub operator: String,
    pub r#type: String,
    pub value: Option<FilterValue>,
}

impl Filter {
    pub fn apply(&self, q: ComposableQueryBuilder) -> XResult<ComposableQueryBuilder> {
        if self.value.is_none() {
            return Ok(q);
        }

        if self.name.is_dimensions() {
            self.filter_dimensions(q)
        } else if self.name.is_numerical() {
            self.filter_numerical(q)
        } else if self.name.is_string() {
            self.filter_string(q)
        } else if self.name.is_boolean() {
            self.filter_bool(q)
        } else {
            Err(XError::InvalidColumnType(self.name))
        }
    }

    fn get_range_values(&self) -> XResult<(f64, f64)> {
        if let Some(FilterValue::Map(f)) = &self.value {
            let start = f
                .get("start")
                .ok_or(XError::Other(format!("Expected start in {:?}", self.value)))?;
            let end = f
                .get("end")
                .ok_or(XError::Other(format!("Expected end in {:?}", self.value)))?;

            if let FilterValue::Number(mut start) = start {
                if let FilterValue::Number(mut end) = end {
                    if self.name.is_cents() {
                        start *= 100.0;
                        end *= 100.0;
                    }

                    return Ok((start, end));
                }
            }

            Err(XError::Other(format!(
                "could not parse start and end numbers from {:?} and {:?}",
                start, end
            )))
        } else {
            Err(XError::Other(format!(
                "Expected FilterValue::Map, got {:?}",
                self.value
            )))
        }
    }

    fn get_f64(&self) -> XResult<f64> {
        if let Some(FilterValue::Number(mut n)) = &self.value {
            if self.name.is_cents() {
                n *= 100.0;
            }

            Ok(n)
        } else {
            Err(XError::Other(format!(
                "Expected FilterValue::Number, got {:?}",
                self.value
            )))
        }
    }

    fn get_bool(&self) -> XResult<bool> {
        if let Some(FilterValue::Boolean(n)) = &self.value {
            Ok(*n)
        } else {
            Err(XError::Other(format!(
                "Expected FilterValue::Bool, got {:?}",
                self.value
            )))
        }
    }

    fn filter_numerical(&self, mut q: ComposableQueryBuilder) -> XResult<ComposableQueryBuilder> {
        let col = self.name.sql_column();
        match self.operator.as_str() {
            "inrange" => {
                let (start, end) = self.get_range_values()?;
                // inrange is inclusive
                q = q.where_clause(format!("{} >= ?", col), start);
                q = q.where_clause(format!("{} <= ?", col), end);
                Ok(q)
            }
            "notinrange" => {
                let (start, end) = self.get_range_values()?;
                // inrange is inclusive, so notinrange is exclusive
                q = q.where_clause(format!("{} < ?", col), start);
                q = q.where_clause(format!("{} > ?", col), end);
                Ok(q)
            }
            "eq" => {
                let v = self.get_f64()?;
                Ok(q.where_clause(format!("{} = ?", col), v))
            }
            "neq" => {
                let v = self.get_f64()?;
                Ok(q.where_clause(format!("{} != ?", col), v))
            }
            "gt" => {
                let v = self.get_f64()?;
                Ok(q.where_clause(format!("{} > ?", col), v))
            }
            "gte" => {
                let v = self.get_f64()?;
                Ok(q.where_clause(format!("{} >= ?", col), v))
            }
            "lt" => {
                let v = self.get_f64()?;
                Ok(q.where_clause(format!("{} < ?", col), v))
            }
            "lte" => {
                let v = self.get_f64()?;
                Ok(q.where_clause(format!("{} <= ?", col), v))
            }
            _ => Err(XError::InvalidOperator(self.operator.clone())),
        }
    }

    fn get_strings(&self) -> XResult<&Vec<String>> {
        if let Some(FilterValue::ArrayString(n)) = &self.value {
            Ok(n)
        } else {
            Err(XError::Other(format!(
                "Expected FilterValue::ArrayString, got {:?}",
                self.value
            )))
        }
    }

    fn get_string(&self) -> XResult<&String> {
        if let Some(FilterValue::String(n)) = &self.value {
            Ok(n)
        } else {
            Err(XError::Other(format!(
                "Expected FilterValue::String, got {:?} for col {:?}",
                self.value, self.name,
            )))
        }
    }

    fn filter_string(&self, mut q: ComposableQueryBuilder) -> XResult<ComposableQueryBuilder> {
        let col = self.name.sql_column();

        println!("{:?}", self);

        if &self.r#type == "select" && self.operator == "inlist" {
            let v = self.get_strings()?;
            for (i, v) in v.iter().enumerate() {
                match i {
                    0 => {
                        // The first one is a normal "and" where clause
                        q = q.where_clause(format!("{} = ?", col), v.to_string());
                    }
                    _ => {
                        // And all subsequent ones are "or" where clauses
                        q = q.or_where(format!("{} = ?", col), v.to_string());
                    }
                }
            }
            return Ok(q);
            // (string) (len=75) "SELECT * FROM \"test\" WHERE x = 1 OR x = 3 OR x = 5 ORDER BY \"test\". LIMIT 1"
        }

        let mut v = self.get_string()?.clone();

        // only allow empty string for empty and nonempty
        let op = self.operator.to_lowercase();
        match op.as_ref() {
            "empty" | "notempty" => {
                v = String::new();
            }
            _ if v.is_empty() => {
                return Ok(q);
            }
            _ => {}
        }

        match op.as_ref() {
            "eq" => Ok(q.where_clause(format!("{col} = ?"), v)),
            "neq" => Ok(q.where_clause(format!("{col} != ?"), v)),
            "contains" => Ok(q.where_clause(format!("{col} ILIKE ?"), format!("%{v}%"))),
            "notcontains" => Ok(q.where_clause(format!("{col} NOT ILIKE ?"), format!("%{v}%"))),
            "empty" => Ok(q.where_clause(format!("{col} = ?"), "".to_string())),
            "notempty" => Ok(q.where_clause(format!("{col} != ?"), "".to_string())),
            "startswith" => Ok(q.where_clause(format!("{col} ILIKE ?"), format!("{v}%"))),
            "endswith" => Ok(q.where_clause(format!("{col} ILIKE ?"), format!("%{v}"))),
            _ => Err(XError::InvalidOperator(self.operator.clone())),
        }
    }

    fn filter_bool(&self, q: ComposableQueryBuilder) -> XResult<ComposableQueryBuilder> {
        let col = self.name.sql_column();
        let v = self.get_bool()?;

        Ok(q.where_clause(format!("{} = ?", col), v))
    }

    fn filter_dimensions(&self, mut q: ComposableQueryBuilder) -> XResult<ComposableQueryBuilder> {
        let col = self.name.sql_column();
        let package_name = format!("package_{}", col);
        let item_name = format!("item_{}", col);
        println!("{:?} {:?}", package_name, item_name);

        match self.operator.as_str() {
            "inrange" => {
                let (start, end) = self.get_range_values()?;
                return Ok(q.multi_where(format!("(({package_name} between ? and ?) or ({item_name} between ? and ? and {package_name} = ?))"), vec![
                    start.into(),
                    end.into(),
                    start.into(),
                    end.into(),
                    0.into(),
                ]).multi_where(format!("({package_name} != ? AND {item_name} != ?)"), vec![0.into(), 0.into()]));
            }
            "notinrange" => {
                let (start, end) = self.get_range_values()?;
                return Ok(q.multi_where(format!("(({package_name} not between ? and ?) or ({item_name} not between ? and ? and {package_name} = ?))"), vec![
                    start.into(),
                    end.into(),
                    start.into(),
                    end.into(),
                    0.into(),
                ]).multi_where(format!("({package_name} != ? AND {item_name} != ?)"), vec![0.into(), 0.into()]));
            }
            _ => {}
        }

        let v = self.get_f64()?;

        let vv0 = vec![v.into(), v.into(), 0.into()];
        q = match self.operator.as_str() {
            "eq" => q.multi_where(
                format!("({package_name} = ? or ({item_name} = ? and {package_name} = ?))"),
                vv0,
            ),
            "neq" => q.multi_where(
                format!("({package_name} != ? or ({item_name} != ? and {package_name} = ?))"),
                vv0,
            ),
            "gt" => q.multi_where(
                format!("({package_name} > ? or ({item_name} > ? and {package_name} = ?))"),
                vv0,
            ),
            "gte" => q.multi_where(
                format!("({package_name} >= ? or ({item_name} >= ? and {package_name} = ?))"),
                vv0,
            ),
            "lt" => q.multi_where(
                format!("({package_name} < ? or ({item_name} < ? and {package_name} = ?))"),
                vv0,
            ),
            "lte" => q.multi_where(
                format!("({package_name} <= ? or ({item_name} <= ? and {package_name} = ?))"),
                vv0,
            ),
            _ => {
                return Err(XError::InvalidOperator(self.operator.clone()));
            }
        };

        if v != 0.0 {
            q = q.multi_where(
                format!("({package_name} != ? OR {item_name} != ?)"),
                vec![0.into(), 0.into()],
            );
        }

        Ok(q)
    }
}

pub trait ColumnHelpers {
    fn is_numerical(&self) -> bool;
    fn is_boolean(&self) -> bool;
    fn is_string(&self) -> bool;
    fn is_cents(&self) -> bool;
    fn is_dimensions(&self) -> bool;
    fn get_type(&self) -> ColType;
}

#[derive(Debug, PartialEq)]
pub enum ColType {
    Numeric,
    String,
    Bool,
    NonFilterable,
}

impl ColumnHelpers for XLSXColumn {
    fn is_numerical(&self) -> bool {
        self.get_type() == ColType::Numeric
    }

    fn is_boolean(&self) -> bool {
        self.get_type() == ColType::Bool
    }

    fn is_string(&self) -> bool {
        self.get_type() == ColType::String
    }

    fn is_cents(&self) -> bool {
        match self {
            XLSXColumn::Checkbox => false,
            XLSXColumn::Notes => false,
            XLSXColumn::Chart => false,
            XLSXColumn::Flags => false,
            XLSXColumn::Errors => false,
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
            XLSXColumn::BuyboxPrice => true,
            XLSXColumn::BuyboxPriceNew => true,
            XLSXColumn::BuyboxPriceUsed => true,
            XLSXColumn::TotalOffersCount => false,
            XLSXColumn::Rank => false,
            XLSXColumn::Category => false,
            XLSXColumn::IsTopLevelCategory => false,
            XLSXColumn::BsrPercentage => true,
            XLSXColumn::Width => false,
            XLSXColumn::Height => false,
            XLSXColumn::Length => false,
            XLSXColumn::Weight => false,
            XLSXColumn::CompetitiveSellers => false,
            XLSXColumn::FinancialsInboundShipping => true,
            XLSXColumn::FinancialsPrepCost => true,
            XLSXColumn::FinancialsFbaStorageFees => true,
            XLSXColumn::AmazonFeesPerItemFee => true,
            XLSXColumn::AmazonFeesFbaFees => true,
            XLSXColumn::AmazonFeesVariableClosingFee => true,
            XLSXColumn::AmazonFeesReferralFee => true,
            XLSXColumn::FinancialsNetRevenue => true,
            XLSXColumn::FinancialsProfit => true,
            // It seems weight to have margin and ROI set to cents
            // where, but this is really just controlling the
            // multiplier.
            XLSXColumn::FinancialsMargin => true,
            XLSXColumn::FinancialsRoi => true,
            XLSXColumn::InputIdentifier => false,
            XLSXColumn::InputCost => true,
            XLSXColumn::InputDiscountPerProduct => true,
            XLSXColumn::InputDiscountCost => true,
            XLSXColumn::InputTotalCogs => true,
            XLSXColumn::InputStock => false,
            XLSXColumn::InputMap => true,
            XLSXColumn::InputSupplierTitle => false,
            XLSXColumn::InputSupplierSku => false,
            XLSXColumn::InputSupplierImage => false,
            XLSXColumn::InputSupplierPackQuantity => false,
            XLSXColumn::SizeTier => false,
            XLSXColumn::LowestPriceNewFba => true,
            XLSXColumn::LowestPriceUsedFba => true,
            XLSXColumn::LowestPriceNewFbm => true,
            XLSXColumn::LowestPriceUsedFbm => true,
            XLSXColumn::NewFbaOffersCount => false,
            XLSXColumn::NewFbmOffersCount => false,
            XLSXColumn::IsBrandBlocklisted => false,
            XLSXColumn::IsMeltable => false,
            XLSXColumn::AddToAmazonLink => false,
            XLSXColumn::InputCustom(_) => false,
        }
    }

    fn is_dimensions(&self) -> bool {
        match self {
            XLSXColumn::Checkbox => false,
            XLSXColumn::Notes => false,
            XLSXColumn::Chart => false,
            XLSXColumn::Flags => false,
            XLSXColumn::Errors => false,
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
            XLSXColumn::Width => true,
            XLSXColumn::Height => true,
            XLSXColumn::Length => true,
            XLSXColumn::Weight => true,
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
            XLSXColumn::InputIdentifier => false,
            XLSXColumn::InputCost => false,
            XLSXColumn::InputDiscountPerProduct => false,
            XLSXColumn::InputDiscountCost => false,
            XLSXColumn::InputTotalCogs => false,
            XLSXColumn::InputStock => false,
            XLSXColumn::InputMap => false,
            XLSXColumn::InputSupplierTitle => false,
            XLSXColumn::InputSupplierSku => false,
            XLSXColumn::InputSupplierImage => false,
            XLSXColumn::InputSupplierPackQuantity => false,
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
            XLSXColumn::InputCustom(_) => false,
        }
    }

    fn get_type(&self) -> ColType {
        use ColType::*;

        match self {
            XLSXColumn::Checkbox => Bool,
            XLSXColumn::Notes => NonFilterable,
            XLSXColumn::Chart => NonFilterable,
            XLSXColumn::Flags => NonFilterable,
            XLSXColumn::Errors => NonFilterable,
            XLSXColumn::Asin => String,
            XLSXColumn::AmazonTitle => String,
            XLSXColumn::FbaEligibility => Bool,
            XLSXColumn::Images => NonFilterable,
            XLSXColumn::SizeName => String,
            XLSXColumn::Brand => String,
            XLSXColumn::Color => String,
            XLSXColumn::ParentAsin => String,
            XLSXColumn::VariationsList => NonFilterable,
            XLSXColumn::NumberOfVariations => Numeric,
            XLSXColumn::AmazonPackQuantity => Numeric,
            XLSXColumn::SmallAndLightEligible => Bool,
            XLSXColumn::SmallAndLightEligibleReasons => String,
            XLSXColumn::BuyboxPrice => Numeric,
            XLSXColumn::BuyboxPriceNew => Numeric,
            XLSXColumn::BuyboxPriceUsed => Numeric,
            XLSXColumn::TotalOffersCount => Numeric,
            XLSXColumn::Rank => Numeric,
            XLSXColumn::Category => String,
            XLSXColumn::IsTopLevelCategory => Bool,
            XLSXColumn::BsrPercentage => Numeric,
            XLSXColumn::Width => Numeric,
            XLSXColumn::Height => Numeric,
            XLSXColumn::Length => Numeric,
            XLSXColumn::Weight => Numeric,
            XLSXColumn::CompetitiveSellers => Numeric,
            XLSXColumn::FinancialsInboundShipping => Numeric,
            XLSXColumn::FinancialsPrepCost => Numeric,
            XLSXColumn::FinancialsFbaStorageFees => Numeric,
            XLSXColumn::AmazonFeesPerItemFee => Numeric,
            XLSXColumn::AmazonFeesFbaFees => Numeric,
            XLSXColumn::AmazonFeesVariableClosingFee => Numeric,
            XLSXColumn::AmazonFeesReferralFee => Numeric,
            XLSXColumn::FinancialsNetRevenue => Numeric,
            XLSXColumn::FinancialsProfit => Numeric,
            XLSXColumn::FinancialsMargin => Numeric,
            XLSXColumn::FinancialsRoi => Numeric,
            XLSXColumn::InputIdentifier => String,
            XLSXColumn::InputCost => Numeric,
            XLSXColumn::InputDiscountPerProduct => Numeric,
            XLSXColumn::InputDiscountCost => Numeric,
            XLSXColumn::InputTotalCogs => Numeric,
            XLSXColumn::InputStock => Numeric,
            XLSXColumn::InputMap => Numeric,
            XLSXColumn::InputSupplierTitle => String,
            XLSXColumn::InputSupplierSku => String,
            XLSXColumn::InputSupplierImage => NonFilterable,
            XLSXColumn::InputSupplierPackQuantity => Numeric,
            XLSXColumn::SizeTier => String,
            XLSXColumn::LowestPriceNewFba => Numeric,
            XLSXColumn::LowestPriceUsedFba => Numeric,
            XLSXColumn::LowestPriceNewFbm => Numeric,
            XLSXColumn::LowestPriceUsedFbm => Numeric,
            XLSXColumn::NewFbaOffersCount => Numeric,
            XLSXColumn::NewFbmOffersCount => Numeric,
            XLSXColumn::IsBrandBlocklisted => Bool,
            XLSXColumn::IsMeltable => Bool,
            XLSXColumn::AddToAmazonLink => NonFilterable,
            XLSXColumn::InputCustom(_) => NonFilterable,
        }
    }
}

#[cfg(test)]
mod filter_tests {
    use crate::result_request::filter::Filter;
    use crate::result_request::FilterValue;
    use crate::xlsx::columns::XLSXColumn::{BuyboxPrice, SizeTier};

    #[test]
    fn it_parses() {
        let s = r#"
        {
            "name": "buybox_price",
            "operator": "inrange",
            "type": "number",
            "value": {
                "start": 112,
                "end": 0
            }
        }
        "#;

        let f = serde_json::from_str::<Filter>(s).unwrap();
        println!("{:?}", f);

        assert_eq!(
            Filter {
                name: BuyboxPrice,
                operator: "inrange".to_string(),
                r#type: "number".to_string(),
                value: Some(FilterValue::Map(
                    vec![
                        ("start".to_string(), FilterValue::Number(112.0)),
                        ("end".to_string(), FilterValue::Number(0.0)),
                    ]
                    .into_iter()
                    .collect()
                ))
            },
            f
        );
    }

    #[test]
    fn it_parses2() {
        let s = r#"
        {
          "name": "buybox_price",
          "operator": "gt",
          "type": "number",
          "value": 1.12
        }
        "#;

        let f = serde_json::from_str::<Filter>(s).unwrap();
        println!("{:?}", f);

        assert_eq!(
            Filter {
                name: BuyboxPrice,
                operator: "gt".to_string(),
                r#type: "number".to_string(),
                value: Some(FilterValue::Number(1.12)),
            },
            f
        );
    }

    #[test]
    fn it_parses3() {
        let s = r#"
        {
            "name": "size_tier",
            "operator": "inlist",
            "type": "select",
            "value": [
                "Small Standard",
                "Large Standard",
                "Large Oversize"
            ]
        }
        "#;

        let f = serde_json::from_str::<Filter>(s).unwrap();
        println!("{:?}", f);

        assert_eq!(
            Filter {
                name: SizeTier,
                operator: "inlist".to_string(),
                r#type: "select".to_string(),
                value: Some(FilterValue::ArrayString(vec![
                    "Small Standard".to_string(),
                    "Large Standard".to_string(),
                    "Large Oversize".to_string(),
                ])),
            },
            f
        );
    }
}
