use crate::error::XResult;
use crate::product::Product;
use crate::result_request::ResultRequest;
use crate::xlsx::columns::XLSXColumn;
use crate::xlsx::format::col_visitor::ColVisitor;
use crate::xlsx::format::Formatters;
use crate::xlsx::formula::{FormulaBuilder, Sheet};
use crate::xlsx::options_builder::OptionsBuilder;
use rs_models::Scan;
use sp_api::marketplaces::Marketplace;
use tracing::info;
use xlsxwriter::prelude::RowColOptions;
use xlsxwriter::{Workbook, Worksheet};

pub struct XlsxBuilder {
    req: ResultRequest,
    formatters: Formatters,
    items: Vec<Product>,
    m: Marketplace,
    workbook: Workbook,
    formulas: FormulaBuilder,
    scan: Scan,
}

impl XlsxBuilder {
    pub fn new(
        path: &str,
        m: Marketplace,
        req: ResultRequest,
        items: Vec<Product>,
        scan: Scan,
    ) -> XResult<XlsxBuilder> {
        Ok(Self {
            workbook: Workbook::new(path)?,
            formatters: Formatters::new(m),
            items,
            m,
            formulas: FormulaBuilder::new(Self::product_cols(&req), Self::error_cols(&req))?,
            req,
            scan,
        })
    }

    pub fn build_products_sheet(self) -> XResult<Self> {
        info!("Building products sheet");
        let items_without_errs = self
            .items
            .iter()
            .filter(|x| !x.has_errors())
            .collect::<Vec<&Product>>();

        let mut sheet = self.workbook.add_worksheet(Some("Scan"))?;

        let cols = Self::product_cols(&self.req);
        let hidden = self.hidden_cols(&cols);

        info!("Adding headers");
        self.add_headers(&cols, &mut sheet)?;
        info!("Adding items");
        self.add_items(&cols, Sheet::Products, &mut sheet, &items_without_errs)?;
        info!("Hiding hidden cols");
        self.hide_hidden_cols(&hidden, &mut sheet)?;

        Ok(self)
    }

    pub fn build_errors_sheet(self) -> XResult<Self> {
        info!("Building errors sheet");
        let items_with_errs = self
            .items
            .iter()
            .filter(|x| x.has_errors())
            .collect::<Vec<&Product>>();

        let mut sheet = self.workbook.add_worksheet(Some("Errors"))?;

        let cols = Self::error_cols(&self.req);
        let hidden = self.hidden_cols(&cols);

        self.add_headers(&cols, &mut sheet)?;
        self.add_items(&cols, Sheet::Errors, &mut sheet, &items_with_errs)?;
        self.hide_hidden_cols(&hidden, &mut sheet)?;

        Ok(self)
    }

    pub fn build_options_sheet(self) -> XResult<Self> {
        info!("Building products sheet");
        OptionsBuilder::new(
            &self.workbook,
            &self.scan,
            self.m,
            &self.formatters,
            &self.formulas,
        )?
        .build()?;

        Ok(self)
    }

    pub fn close(self) -> XResult<()> {
        info!("Closing workbook");
        self.workbook.close()?;
        Ok(())
    }

    fn add_headers(&self, cols: &[XLSXColumn], sheet: &mut Worksheet) -> XResult<()> {
        for (col, c) in cols.iter().enumerate() {
            let f = self.formatters.header(*c);
            sheet.write_string(0, col as u16, c.header(&self.scan), f)?;

            {
                use crate::xlsx::columns::XLSXColumn::*;
                if matches!(c, Height | Length | Width) {
                    sheet.write_comment(0, col as u16, "In inches")?;
                } else if matches!(c, Weight) {
                    sheet.write_comment(0, col as u16, "In pounds")?;
                }
            }
        }

        Ok(())
    }

    fn add_items(
        &self,
        cols: &[XLSXColumn],
        sheet_type: Sheet,
        sheet: &mut Worksheet,
        items: &[&Product],
    ) -> XResult<()> {
        for (row, item) in items.iter().enumerate() {
            // Rows are 0-indexed, but the first is taken up with the header. So we add 1.
            let visitor = ColVisitor::new(
                self.m,
                &self.formatters,
                &self.formulas,
                sheet_type,
                cols,
                row + 1,
            );
            visitor.write(sheet, item)?;
        }

        Ok(())
    }

    /// Hides the columns at the given indexes
    fn hide_hidden_cols(&self, hidden: &[usize], sheet: &mut Worksheet) -> XResult<()> {
        for i in hidden.iter().map(|x| *x as u16) {
            sheet.set_column_opt(i, i, 8.43, None, &RowColOptions::new(true, 0, false))?;
        }

        Ok(())
    }

    fn product_cols(req: &ResultRequest) -> Vec<XLSXColumn> {
        let mut visible_cols: Vec<XLSXColumn> = req
            .order
            .clone()
            .into_iter()
            .filter(|c| c.visible_on_products_sheet())
            .collect();

        Self::add_extra_cols(&mut visible_cols);
        visible_cols
    }

    fn error_cols(req: &ResultRequest) -> Vec<XLSXColumn> {
        let mut visible_cols: Vec<XLSXColumn> = req
            .order
            .clone()
            .into_iter()
            .filter(|c| c.visible_on_errors_sheet())
            .collect();

        Self::add_extra_cols(&mut visible_cols);
        visible_cols.insert(0, XLSXColumn::Errors);
        visible_cols
    }

    /// Returns the indexes of the columns that should be hidden.
    fn hidden_cols(&self, cols: &[XLSXColumn]) -> Vec<usize> {
        Vec::from_iter(
            cols.iter()
                .enumerate()
                .map(|x| (x.0, self.req.visible.get(x.1).unwrap_or(&false)))
                .filter(|x| !*x.1)
                .map(|x| x.0),
        )
    }

    fn add_extra_cols(cols: &mut Vec<XLSXColumn>) {
        // Add the extra columns used just in the exports. These are generally to show stuff
        // that would be shown behind a tooltip on the app.
        let mut add_after = |add: XLSXColumn, find: XLSXColumn| {
            let i = cols
                .iter()
                .enumerate()
                .find(|(_, c)| **c == find)
                .map(|(i, _)| i);

            if let Some(i) = i {
                cols.insert(i + 1, add);
            }
        };
        add_after(
            XLSXColumn::SmallAndLightEligibleReasons,
            XLSXColumn::SmallAndLightEligible,
        );
    }
}
