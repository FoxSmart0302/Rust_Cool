pub mod config;
mod formula_item;

use crate::error::{XError, XResult};
use crate::xlsx::columns::XLSXColumn;
use crate::xlsx::columns::XLSXColumn::AmazonPackQuantity;
use crate::xlsx::coords::Coords;
use crate::xlsx::formula::formula_item::FormulaItem;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub const INBOUND_SHIPPING: &str = r#"IF(`Weight` <> "", `Weight` * `Config::InboundShipping`, 0)"#;
pub const PREP_COST: &str = r#"IF(
    `Amazon Pack Quantity` = 1, 
    `Config::PrepSingle`, 
    IF(
        `Amazon Pack Quantity` <= `Config::PrepMultiThreshold`,
        `Config::PrepMultiBase`,
        `Config::PrepMultiBase` + (`Amazon Pack Quantity` - `Config::PrepMultiThreshold`) * `Config::PrepMultiAdtlCostPerUnit`
    )
)"#;
pub const NET_REVENUE: &str = r#"`Buybox Price`
    - (`Buybox Price` - (`Buybox Price` / (1 + `Config::AmazonVat`)))
    - `Per Item Fee`
    - `FBA Fees` 
    - `Variable Closing Fee` 
    - `Referral Fee`"#;
pub const PROFIT: &str = r#"IF(`Buybox Price` <> 0,
    `Net Revenue` - `Total COGS` - `Inbound Shipping` - `Prep Cost` - `FBA Storage Fees`,
    ""
)"#;
pub const MARGIN: &str =
    r#"IF(AND(`Buybox Price` <> "", `Buybox Price` <> 0),`Profit`/`Buybox Price`, "")"#;
pub const ROI: &str = r#"IF(AND(`Profit` <> "", `Total COGS` <> "", `Total COGS` <> 0), `Profit` / `Total COGS`, "")"#;
pub const DISCOUNT_COST: &str = r#"`Cost` * (1-`Config::SupplierDiscount`) * (1-`Product Discount`) * (1+`Config::SupplierVat`)"#;
pub const TOTAL_COGS: &str =
    r#"`Adjusted Cost` * `Amazon Pack Quantity` / `Supplier Pack Quantity`"#;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
}

#[cfg(test)]
fn formulas() -> Vec<&'static str> {
    vec![
        INBOUND_SHIPPING,
        PREP_COST,
        NET_REVENUE,
        PROFIT,
        MARGIN,
        ROI,
        DISCOUNT_COST,
        TOTAL_COGS,
    ]
}

pub struct FormulaBuilder {
    product_cols: HashMap<XLSXColumn, usize>,
    err_cols: HashMap<XLSXColumn, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sheet {
    Products,
    Errors,
}

impl FormulaBuilder {
    pub fn new(product_cols: Vec<XLSXColumn>, err_cols: Vec<XLSXColumn>) -> XResult<Self> {
        let s = Self {
            product_cols: product_cols
                .into_iter()
                .enumerate()
                .map(|(i, c)| (c, i))
                .collect(),
            err_cols: err_cols
                .into_iter()
                .enumerate()
                .map(|(i, c)| (c, i))
                .collect(),
        };

        Ok(s)
    }

    /// Prepares a templated formula for use. It replaces templated column
    /// names with the actual A1 coordinates of the columns.
    pub fn parse(&self, formula: &str, row: usize, sheet: Sheet) -> XResult<String> {
        let matches = self.extract_replacements(formula)?;

        // Perform replacements
        let mut formula = formula.to_string();
        for col in matches.into_iter() {
            match col.try_into()? {
                FormulaItem::Column(c) => {
                    let coords = match self.get_coordinates(sheet, c, row) {
                        Ok(coords) => coords,
                        Err(e) => {
                            if c == AmazonPackQuantity && sheet == Sheet::Errors {
                                // hard code to one
                                let col = format!("`{}`", col);
                                formula = formula.replace(&col, "1");
                                continue;
                            } else {
                                println!("Error getting coordinates for col {:?}: {:?}", c, e);
                                println!("Formula: {}", formula);
                                println!("Sheet: {:?}", sheet);
                                return Err(e);
                            }
                        }
                    };
                    let coords = coords.to_a1_coords();
                    let col = format!("`{}`", col);
                    formula = formula.replace(&col, &coords);
                }
                FormulaItem::Option(o) => {
                    let coords = format!("Options!{}", o.coords());
                    let col = format!("`{}`", col);
                    formula = formula.replace(&col, &coords);
                }
            }
        }

        // Cleanup whitespace
        let formula = WHITESPACE_REGEX.replace_all(&formula, " ");
        let formula = formula.replace(" )", ")");
        let formula = formula.replace("( ", "(");

        Ok(formula)
    }

    /// Returns a list of all items that need dynamic replacement.
    ///
    /// # Example
    /// ```ignore
    /// let formula = "IF(`Weight` <> "", `Weight` * `Config::InboundShipping`, 0)";
    /// let replacements = self.extract_replacements(formula)?;
    /// assert_eq!(replacements, vec!["Weight", "Config::InboundShipping"]);
    /// ```
    fn extract_replacements<'a>(&self, formula: &'a str) -> XResult<HashSet<&'a str>> {
        let mut matches = HashSet::new();
        let mut found = false;
        let mut start = 0;
        for (i, char) in formula.chars().enumerate() {
            if char == '`' {
                if found {
                    let end = i;
                    let col = &formula[start + 1..end];
                    matches.insert(col);
                    found = false;
                } else {
                    found = true;
                    start = i;
                }
            }
        }
        if found {
            return Err(XError::UnterminatedFormula(formula.to_string()));
        }

        Ok(matches)
    }

    /// Returns the (row,col) coords for the given column
    fn get_coordinates(&self, sheet: Sheet, col: XLSXColumn, row: usize) -> XResult<Coords> {
        let col = self.get_column_index(sheet, col)?;
        Ok(Coords {
            row: row as u32,
            col: col as u16,
        })
    }

    /// Returns the column index for the given column
    fn get_column_index(&self, sheet: Sheet, col: XLSXColumn) -> XResult<usize> {
        match sheet {
            Sheet::Products => self.product_cols.get(&col).copied(),
            Sheet::Errors => self.err_cols.get(&col).copied(),
        }
        .ok_or_else(|| XError::UnhandledFormulaColumn(col))
    }
}

#[cfg(test)]
mod formula_tests {
    use crate::error::XResult;
    use crate::result_request::ResultRequest;
    use crate::xlsx::columns::XLSXColumn;
    use crate::xlsx::formula::{formulas, FormulaBuilder, Sheet};
    use std::io::Read;

    /// A very important sanity check! Ensures that the dynamic replacement of all
    /// the formulas works as expectd.
    #[test]
    fn it_works() -> XResult<()> {
        // Print pwd
        let mut f = std::fs::File::open("fixtures/request.json")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let req: ResultRequest = serde_json::from_str(&s)?;

        // let mut all: HashSet<Column> = HashSet::from_iter(Column::iter());
        // for c in &req.order {
        //     all.remove(c);
        // }

        let product_cols: Vec<XLSXColumn> = req
            .order
            .clone()
            .into_iter()
            .filter(|c| c.visible_on_products_sheet())
            .collect();
        let err_cols: Vec<XLSXColumn> = req
            .order
            .clone()
            .into_iter()
            .filter(|c| c.visible_on_errors_sheet())
            .collect();
        let f = FormulaBuilder::new(product_cols, err_cols)?;

        for formula in formulas() {
            f.parse(formula, 1, Sheet::Products)?;
        }

        Ok(())
    }
}
