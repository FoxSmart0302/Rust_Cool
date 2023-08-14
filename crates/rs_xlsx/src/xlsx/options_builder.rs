use crate::error::XResult;
use crate::xlsx::coords::Coords;
use crate::xlsx::format::numbers::{NUMBER, PERCENT};
use crate::xlsx::format::{Formatters, BLUE_DARK, BLUE_LIGHT, BLUE_MID};
use crate::xlsx::formula::config::Config;
use crate::xlsx::formula::{FormulaBuilder, Sheet, PREP_COST};
use crate::xlsx::marketplace_helper::MarketplaceHelper;
use rs_models::Scan;
use sp_api::marketplaces::Marketplace;
use xlsxwriter::format::{FormatAlignment, FormatBorder, FormatColor};
use xlsxwriter::prelude::GridLines;
use xlsxwriter::{Format, Workbook, Worksheet};

pub struct OptionsBuilder<'a> {
    sheet: Worksheet<'a>,
    formulas: &'a FormulaBuilder,
    scan: &'a Scan,
    money: Format,
    percent: Format,
    number: Format,
    number_dark: Format,
}

impl<'a> OptionsBuilder<'a> {
    pub fn new(
        workbook: &'a Workbook,
        scan: &'a Scan,
        m: Marketplace,
        formatters: &Formatters,
        formulas: &'a FormulaBuilder,
    ) -> XResult<Self> {
        let mut sheet = workbook.add_worksheet(Some("Options"))?;
        sheet.gridlines(GridLines::HideAllGridLines);

        let f = m.money_builder().default_zero().build();
        let money = formatters
            .build_cell((0, f.as_str()))
            .set_bg_color(FormatColor::Custom(BLUE_LIGHT))
            .to_owned();
        let percent = formatters
            .build_cell((0, PERCENT))
            .set_bg_color(FormatColor::Custom(BLUE_LIGHT))
            .to_owned();
        let number = formatters
            .build_cell((0, NUMBER))
            .set_bg_color(FormatColor::Custom(BLUE_LIGHT))
            .to_owned();
        let number_dark = formatters
            .build_cell((0, NUMBER))
            .set_bg_color(FormatColor::Custom(BLUE_MID))
            .to_owned();

        Ok(Self {
            sheet,
            scan,
            money,
            percent,
            number,
            number_dark,
            formulas,
        })
    }

    pub fn build(self) -> XResult<()> {
        self.set_widths()?
            .set_options()?
            .set_inbound_shipping_rate()?
            .set_supplier_discount()?
            .set_supplier_vat()?
            .set_amazon_vat()?
            .set_prep_costs()?
            .set_prep_cost_single_unit()?
            .set_prep_cost_multi_base()?
            .set_prep_cost_multi_threshold()?
            .set_prep_cost_multi_additional_cost_per_unit()?
            .set_prep_examples()?;

        Ok(())
    }

    fn set_widths(mut self) -> XResult<Self> {
        self.sheet.set_column(0, 0, 1.67, None)?;
        self.sheet.set_column(1, 1, 17.67, None)?;
        self.sheet.set_column(4, 4, 27.00, None)?;
        Ok(self)
    }

    fn set_options(mut self) -> XResult<Self> {
        let Coords { row, col } = Config::InboundShipping.get_coords()?;
        assert!(row > 0 && col > 0);
        let f = Format::new()
            .set_border_bottom(FormatBorder::Thin)
            .set_bg_color(FormatColor::Custom(BLUE_DARK))
            .set_align(FormatAlignment::Center)
            .set_bold()
            .to_owned();
        self.sheet
            .merge_range(row - 1, col - 1, row - 1, col, "Options", Some(&f))?;
        Ok(self)
    }

    fn set_inbound_shipping_rate(mut self) -> XResult<Self> {
        let coords = Config::InboundShipping.get_coords()?;

        let v = self
            .scan
            .options
            .as_ref()
            .map(|x| x.input_shipping_rate as f64 / 100.0)
            .unwrap_or(0.0);

        write(
            &mut self.sheet,
            coords,
            "Inbound Shipping Rate",
            v,
            Some(&self.money),
        )?;
        Ok(self)
    }

    fn set_supplier_discount(mut self) -> XResult<Self> {
        let coords = Config::SupplierDiscount.get_coords()?;

        let v = self
            .scan
            .options
            .as_ref()
            .map(|x| x.discount_supplier as f64 / 10000.0)
            .unwrap_or(0.0);

        write(
            &mut self.sheet,
            coords,
            "Supplier Discount",
            v,
            Some(&self.percent),
        )?;
        Ok(self)
    }

    fn set_supplier_vat(mut self) -> XResult<Self> {
        let coords = Config::SupplierVat.get_coords()?;

        let v = self
            .scan
            .options
            .as_ref()
            .map(|x| x.supplier_vat.unwrap_or(0) as f64 / 10000.0)
            .unwrap_or(0.0);

        write(
            &mut self.sheet,
            coords,
            "Supplier VAT",
            v,
            Some(&self.percent),
        )?;
        Ok(self)
    }

    fn set_amazon_vat(mut self) -> XResult<Self> {
        let coords = Config::AmazonVat.get_coords()?;

        let v = self
            .scan
            .options
            .as_ref()
            .map(|x| x.amazon_vat.unwrap_or(0) as f64 / 10000.0)
            .unwrap_or(0.0);

        write(
            &mut self.sheet,
            coords,
            "Amazon VAT",
            v,
            Some(&self.percent),
        )?;
        Ok(self)
    }

    fn set_prep_costs(mut self) -> XResult<Self> {
        let Coords { row, col } = Config::PrepSingle.get_coords()?;
        assert!(row > 0 && col > 0);
        let f = Format::new()
            .set_border_bottom(FormatBorder::Thin)
            .set_bg_color(FormatColor::Custom(BLUE_DARK))
            .set_align(FormatAlignment::Center)
            .set_bold()
            .to_owned();
        self.sheet
            .merge_range(row - 1, col - 1, row - 1, col, "Prep Costs", Some(&f))?;
        Ok(self)
    }

    fn set_prep_cost_single_unit(mut self) -> XResult<Self> {
        let coords = Config::PrepSingle.get_coords()?;

        let v = self
            .scan
            .options
            .as_ref()
            .map(|x| x.prep_cost)
            .unwrap_or(0.0);

        write(&mut self.sheet, coords, "Single Unit", v, Some(&self.money))?;
        Ok(self)
    }

    fn set_prep_cost_multi_base(mut self) -> XResult<Self> {
        let coords = Config::PrepMultiBase.get_coords()?;

        let v = self
            .scan
            .options
            .as_ref()
            .map(|x| x.multipack_prep_cost.cost_for_first_n)
            .unwrap_or(0.0);

        write(
            &mut self.sheet,
            coords,
            "Multipack Base Price",
            v,
            Some(&self.money),
        )?;
        Ok(self)
    }

    fn set_prep_cost_multi_threshold(mut self) -> XResult<Self> {
        let coords = Config::PrepMultiThreshold.get_coords()?;

        let v = self
            .scan
            .options
            .as_ref()
            .map(|x| x.multipack_prep_cost.first_n as f64)
            .unwrap_or(0.0);

        write(
            &mut self.sheet,
            coords,
            "Multipack Threshold",
            v,
            Some(&self.number),
        )?;
        Ok(self)
    }

    fn set_prep_cost_multi_additional_cost_per_unit(mut self) -> XResult<Self> {
        let coords = Config::PrepMultiAdtlCostPerUnit.get_coords()?;

        let v = self
            .scan
            .options
            .as_ref()
            .map(|x| x.multipack_prep_cost.cost_for_rest)
            .unwrap_or(0.0);

        write(
            &mut self.sheet,
            coords,
            "Multipack Additional Cost per Unit",
            v,
            Some(&self.money),
        )?;
        Ok(self)
    }

    fn set_prep_examples(mut self) -> XResult<Self> {
        // Build top header
        let Coords { row, col } = Config::PrepMultiAdtlCostPerUnit.get_coords()?;
        let f = Format::new()
            .set_bg_color(FormatColor::Custom(BLUE_DARK))
            .set_align(FormatAlignment::Center)
            .set_bold()
            .to_owned();
        self.sheet.merge_range(
            row + 2,
            col - 1,
            row + 2,
            col,
            "Example Prep Cost Calculations",
            Some(&f),
        )?;

        // Build 2nd row of header
        // No bold + border bottom
        let mut f = Format::new()
            .set_bg_color(FormatColor::Custom(BLUE_DARK))
            .set_align(FormatAlignment::Center)
            .set_border_bottom(FormatBorder::Thin)
            .set_align(FormatAlignment::Right)
            .to_owned();
        self.sheet
            .write_string(row + 3, col - 1, "Amazon Pack Quantity", Some(&f))?;

        f = f.set_align(FormatAlignment::Center).to_owned();
        self.sheet
            .write_string(row + 3, col, "Prep Cost", Some(&f))?;

        // Writ the example Amazon pack quantities
        let mut write_example_pack_qty = |row: u32, num: f64| {
            self.sheet
                .write_number(row, col - 1, num, Some(&self.number_dark))
        };
        write_example_pack_qty(row + 4, 1.0)?;
        write_example_pack_qty(row + 5, 2.0)?;
        write_example_pack_qty(row + 6, 3.0)?;
        write_example_pack_qty(row + 7, 5.0)?;
        write_example_pack_qty(row + 8, 10.0)?;

        let mut add_prep_cost_formula = |row: u32, col: u16| -> XResult<()> {
            let f = PREP_COST.to_string().replace(
                "`Amazon Pack Quantity`",
                &Coords { row, col: col - 1 }.to_a1_coords(),
            );
            let f = self.formulas.parse(&f, row as usize, Sheet::Products)?;
            self.sheet.write_formula(row, col, &f, Some(&self.money))?;
            Ok(())
        };
        add_prep_cost_formula(row + 4, col)?;
        add_prep_cost_formula(row + 5, col)?;
        add_prep_cost_formula(row + 6, col)?;
        add_prep_cost_formula(row + 7, col)?;
        add_prep_cost_formula(row + 8, col)?;

        Ok(self)
    }
}

fn write(
    sheet: &mut Worksheet,
    Coords { row, col }: Coords,
    msg: &str,
    number: f64,
    number_format: Option<&Format>,
) -> XResult<()> {
    assert!(col > 0, "col must be > 0 {}", msg);

    let f = Format::new()
        .set_bg_color(FormatColor::Custom(BLUE_MID))
        .to_owned();
    sheet.write_string(row, col - 1, msg, Some(&f))?;
    sheet.write_number(row, col, number, number_format)?;

    Ok(())
}
