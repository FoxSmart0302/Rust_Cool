use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Scan {
    pub id: i64,
    pub user_id: i64,
    pub account_id: i64,
    // @todo load enum
    pub marketplace_id: i64,
    // @todo load enum
    pub status: i16,
    // @todo load enum
    pub source_type_id: i16,
    pub source_id: i64,
    pub name: String,
    pub products: i32,
    pub errors: i32,
    pub speed: i32,
    pub supplier_file: String,
    pub results: String,
    pub options: Option<Json<ScanOptions>>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub filename: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub lines: i32,
}

impl Scan {
    pub fn custom_col_header(&self, index: u32) -> Option<&str> {
        let header_index = self.options.as_ref()?.custom_columns.get(index as usize)?;

        let header = self
            .options
            .as_ref()?
            .header
            .as_ref()?
            .get(*header_index as usize)?
            .as_str();

        Some(header)
    }
    pub fn get_custom_col_index(&self, col_header: &str) -> Option<u32> {
        let header_index = self
            .options
            .as_ref()?
            .header
            .as_ref()?
            .iter()
            .position(|x| *x == col_header)? as i32;

        let index = self
            .options
            .as_ref()?
            .custom_columns
            .iter()
            .position(|x| *x == header_index)? as u32;

        Some(index)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanOptions {
    pub name: String,
    /// Older scans may not have this field
    pub header: Option<Vec<String>>,
    /// Older scans may not have this field
    pub mapping: Option<ScanMapping>,
    pub prep_cost: f64,
    pub amazon_vat: Option<i32>,
    pub supplier_vat: Option<i32>,
    pub custom_columns: Vec<i32>,
    pub marketplace_id: i32,
    pub discount_supplier: i32,
    /// Older scans may not have this field
    pub multipack_override: Option<bool>,
    pub input_shipping_rate: i32,
    pub multipack_prep_cost: MultipackPrepCost,
    /// Older scans may not have this field
    pub multipack_override_quantity: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanMapping {
    pub id: i32,
    pub map: i32,
    pub cost: i32,
    pub supplier_sku: i32,
    pub custom_columns: Vec<i32>,
    pub stock_quantity: i32,
    pub supplier_image: i32,
    pub supplier_title: i32,
    pub discount_per_product: i32,
    pub supplier_pack_quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MultipackPrepCost {
    pub first_n: i32,
    pub enabled: bool,
    pub cost_for_rest: f64,
    pub cost_for_first_n: f64,
}
