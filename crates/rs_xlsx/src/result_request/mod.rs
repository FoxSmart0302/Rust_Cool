mod filter;

use crate::error::XResult;
use crate::result_request::filter::Filter;
use crate::xlsx::columns::XLSXColumn;
use rs_models::Scan;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// This struct is a raw, unparsed [ResultRequest]. After parsing
/// it will convert the string values in the `order` and `visible`
/// fields into XLSXColumns, if they are valid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawResultRequest {
    pub limit: Option<u64>,
    pub skip: Option<u64>,
    pub sort: Option<String>,
    pub filter: Vec<Filter>,
    // "amazon_title" / "-amazon_title"
    pub list: i64,
    pub order: Vec<String>,
    pub visible: HashMap<String, bool>,
}

impl RawResultRequest {
    pub fn parse(self, scan: &Scan) -> XResult<ResultRequest> {
        let order = self
            .order
            .into_iter()
            .map(|x| XLSXColumn::try_from(x.as_str()))
            .collect::<XResult<Vec<XLSXColumn>>>()?;

        let visible = self
            .visible
            .into_iter()
            .map(|(k, v)| {
                let col = match XLSXColumn::try_from(k.as_str()) {
                    Ok(col) => col,
                    Err(e) => match scan.get_custom_col_index(k.as_str()) {
                        Some(index) => XLSXColumn::InputCustom(index),
                        None => {
                            println!("could not find a custom col index for {}", k);
                            return Err(e);
                        }
                    },
                };

                Ok((col, v))
            })
            .collect::<XResult<HashMap<XLSXColumn, bool>>>()?;

        println!("{:#?}", order);
        Ok(ResultRequest {
            limit: self.limit,
            skip: self.skip,
            sort: self.sort,
            filter: self.filter,
            list: self.list,
            order,
            visible,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultRequest {
    /// The maximum number of items to return. Used for pagination.
    pub limit: Option<u64>,
    /// The offset to start on. Used for pagination.
    pub skip: Option<u64>,
    /// An optional sort field. If the field is prefixed with a
    /// `-` it will be sorted descending.
    pub sort: Option<String>,
    /// An optional life of filters
    pub filter: Vec<Filter>,
    /// Which List this export is for. A value is 0 means not to
    /// use a specific list and instead use all the products.
    pub list: i64,
    /// The order in which the columns will appear. These can be
    /// rearranged by the user in the UI.
    pub order: Vec<XLSXColumn>,
    /// Which columns are visible. These can be toggled by the
    /// user in the UI.
    pub visible: HashMap<XLSXColumn, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FilterValue {
    String(String),
    Number(f64),
    Boolean(bool),
    ArrayString(Vec<String>),
    Map(HashMap<String, FilterValue>),
    Null,
}

#[cfg(test)]
mod request_tests {
    use crate::error::XResult;
    use crate::result_request::{RawResultRequest, ResultRequest};
    use crate::xlsx::columns::XLSXColumn;
    use rs_models::{MultipackPrepCost, Scan, ScanMapping, ScanOptions};
    use sqlx::types::Json;
    use std::io::Read;

    #[test]
    fn it_parses() -> XResult<()> {
        // Load text file from "request.json"
        // Parse it into a Request struct
        // Assert that the struct is correct
        let mut f = std::fs::File::open("fixtures/request.json")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let r = serde_json::from_str::<ResultRequest>(&s)?;
        println!("{:?}", r);

        Ok(())
    }

    #[test]
    fn it_parses_with_a_custom_column() -> XResult<()> {
        // Load text file from "request.json"
        // Parse it into a Request struct
        // Assert that the struct is correct
        let mut f = std::fs::File::open("fixtures/request_with_custom_col.json")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let r = serde_json::from_str::<ResultRequest>(&s)?;
        println!("{:?}", r);

        Ok(())
    }

    #[test]
    fn it_parses_with_custom_col_in_visible() -> XResult<()> {
        let scan = Scan {
            id: 0,
            account_id: 0,
            marketplace_id: 0,
            status: 0,
            source_type_id: 0,
            source_id: 0,
            name: "".to_string(),
            products: 0,
            errors: 0,
            speed: 0,
            supplier_file: "".to_string(),
            results: "".to_string(),
            options: Some(Json(ScanOptions {
                name: "".to_string(),
                header: Some(vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ASIN".to_string(),
                    "MSRP".to_string(),
                ]),
                mapping: Some(ScanMapping {
                    id: 0,
                    map: 0,
                    cost: 0,
                    supplier_sku: 0,
                    custom_columns: vec![3, 4],

                    stock_quantity: 0,
                    supplier_image: 0,
                    supplier_title: 0,
                    discount_per_product: 0,
                    supplier_pack_quantity: 0,
                }),
                prep_cost: 1.0,
                amazon_vat: None,
                supplier_vat: None,
                custom_columns: vec![3, 4],
                marketplace_id: 0,
                discount_supplier: 123,
                multipack_override: None,
                input_shipping_rate: 12,
                multipack_prep_cost: MultipackPrepCost {
                    first_n: 3,
                    enabled: true,
                    cost_for_rest: 0.5,
                    cost_for_first_n: 0.75,
                },
                multipack_override_quantity: None,
            })),
            created_at: None,
            updated_at: None,
            filename: "".to_string(),
            deleted_at: None,
            user_id: 0,
            lines: 0,
        };

        let mut f = std::fs::File::open("fixtures/request_with_custom_col2.json")?;
        // let mut f = std::fs::File::open("fixtures/request_with_custom_col.json")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let r = serde_json::from_str::<RawResultRequest>(&s)?;
        let r = r.parse(&scan)?;

        // ASIN
        assert_eq!(r.visible.get(&XLSXColumn::InputCustom(0)), Some(&true));
        // MSRP
        assert_eq!(r.visible.get(&XLSXColumn::InputCustom(1)), Some(&true));

        Ok(())
    }
}
