use crate::error::XResult;
use crate::product::Product;
use crate::xlsx::columns::XLSXColumn;
use crate::xlsx::format::Formatters;
use crate::xlsx::formula::{FormulaBuilder, Sheet};
use sp_api::marketplaces::Marketplace;
use xlsxwriter::Worksheet;

pub struct ColVisitor<'a> {
    cols: &'a [XLSXColumn],
    row: u32,
    formatters: &'a Formatters,
    formulas: &'a FormulaBuilder,
    m: Marketplace,
    sheet_type: Sheet,
}

impl<'a> ColVisitor<'a> {
    pub fn new(
        m: Marketplace,
        formatters: &'a Formatters,
        formulas: &'a FormulaBuilder,
        sheet_type: Sheet,
        cols: &'a [XLSXColumn],
        row: usize,
    ) -> Self {
        Self {
            cols,
            row: row as u32,
            formatters,
            m,
            formulas,
            sheet_type,
        }
    }

    pub fn write(&self, sheet: &mut Worksheet, item: &Product) -> XResult<()> {
        for (col, c) in self.cols.iter().enumerate() {
            let f = self.formatters.cell(*c);

            match c.formula() {
                Some(formula) => {
                    // info!("Writing formula: {} {:?}", formula, c);
                    let formula =
                        self.formulas
                            .parse(formula, self.row as usize, self.sheet_type)?;

                    sheet.write_formula(self.row, col as u16, &formula, f)?;
                }
                None => {
                    // info!("Writing std col: {:?}", c);
                    c.display(self.m, item)
                        .write(self.row, col as u16, sheet, f)?;
                }
            }
        }

        Ok(())
    }
}
