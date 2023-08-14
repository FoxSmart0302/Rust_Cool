use crate::xlsx::columns::XLSXColumn;
use crate::xlsx::format::colors::next_color;
use crate::xlsx::format::numbers::{NUMBER, PERCENT};
use crate::xlsx::format::{BLUE, GRAY, GREEN, ORANGE, RED};
use crate::xlsx::marketplace_helper::MarketplaceHelper;
use sp_api::marketplaces::Marketplace;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use xlsxwriter::format::{FormatBorder, FormatColor, FormatUnderline};
use xlsxwriter::Format;

/// Holds the [Format]s needed for each column and it's header.
///
/// These are dynamically constructed on initialization for the given [Marketplace].
pub struct Formatters {
    cells: HashMap<XLSXColumn, Format>,
    headers: HashMap<XLSXColumn, Format>,
}

type FormatBuilder<'a> = (u32, &'a str);

impl Formatters {
    pub fn new(m: Marketplace) -> Self {
        use crate::xlsx::columns::XLSXColumn::*;

        let mut s = Self {
            cells: HashMap::new(),
            headers: HashMap::new(),
        };

        let default = Format::new();
        let default_header = Format::new()
            .set_bold()
            .set_border_bottom(FormatBorder::Thin)
            .to_owned();

        let link = Format::new()
            .set_underline(FormatUnderline::Single)
            .set_font_color(FormatColor::Custom(0x1265BE))
            .to_owned();
        let gray_link = link
            .clone()
            .set_bg_color(FormatColor::Custom(GRAY))
            .to_owned();

        for c in XLSXColumn::iter() {
            s.cells.insert(c, default.clone());
            s.headers.insert(c, default_header.clone());
        }

        s.add(Rank, (0, NUMBER));
        s.add(BsrPercentage, (0, PERCENT));

        s.add(BuyboxPrice, (ORANGE, m.currency_formatter()));
        s.add(BuyboxPriceNew, (ORANGE, m.currency_formatter()));
        s.add(BuyboxPriceUsed, (ORANGE, m.currency_formatter()));
        s.add(TotalOffersCount, (ORANGE, NUMBER));
        s.add(CompetitiveSellers, (ORANGE, NUMBER));
        s.add(LowestPriceNewFba, (ORANGE, m.currency_formatter()));
        s.add(LowestPriceUsedFba, (ORANGE, m.currency_formatter()));
        s.add(LowestPriceNewFbm, (ORANGE, m.currency_formatter()));
        s.add(LowestPriceUsedFbm, (ORANGE, m.currency_formatter()));
        s.add(NewFbaOffersCount, (ORANGE, NUMBER));
        s.add(NewFbmOffersCount, (ORANGE, NUMBER));

        s.add(Width, (BLUE, ""));
        s.add(Height, (BLUE, ""));
        s.add(Length, (BLUE, ""));
        s.add(Weight, (BLUE, ""));

        s.add(FinancialsInboundShipping, (BLUE, m.currency_formatter()));
        s.add(FinancialsPrepCost, (BLUE, m.currency_formatter()));
        s.add(FinancialsFbaStorageFees, (BLUE, m.currency_formatter()));
        s.add(SizeTier, (BLUE, NUMBER));

        s.add(AmazonFeesPerItemFee, (GREEN, m.currency_formatter()));
        s.add(AmazonFeesFbaFees, (GREEN, m.currency_formatter()));
        s.add(
            AmazonFeesVariableClosingFee,
            (GREEN, m.currency_formatter()),
        );
        s.add(AmazonFeesReferralFee, (GREEN, m.currency_formatter()));
        s.add(FinancialsNetRevenue, (GREEN, m.currency_formatter()));
        s.add(FinancialsProfit, (GREEN, m.currency_formatter()));
        s.add(FinancialsMargin, (GREEN, PERCENT));
        s.add(FinancialsRoi, (GREEN, PERCENT));

        s.add(InputIdentifier, (GRAY, ""));
        s.add(InputCost, (GRAY, m.currency_formatter()));
        s.add(InputDiscountPerProduct, (GRAY, PERCENT));
        s.add(InputDiscountCost, (GRAY, m.currency_formatter()));
        s.add(InputTotalCogs, (GRAY, m.currency_formatter()));
        s.add(InputStock, (GRAY, NUMBER));
        s.add(InputMap, (GRAY, m.currency_formatter()));
        s.add(InputSupplierTitle, (GRAY, ""));
        s.add(InputSupplierSku, (GRAY, ""));
        s.add(InputSupplierImage, (GRAY, ""));
        s.add(InputSupplierPackQuantity, (GRAY, NUMBER));
        s.add(AddToAmazonLink, (GRAY, ""));

        s.add(Errors, (RED, ""));

        s.cells.insert(Asin, link.clone());
        s.cells.insert(Images, link.clone());
        s.cells.insert(InputSupplierImage, gray_link.clone());
        s.cells.insert(AddToAmazonLink, gray_link.clone());

        s
    }

    /// Returns the cell format for the given column.
    pub fn cell(&self, c: XLSXColumn) -> Option<&Format> {
        self.cells.get(&c)
    }

    /// Returns the header format for the given column.
    pub fn header(&self, c: XLSXColumn) -> Option<&Format> {
        self.headers.get(&c)
    }

    /// Adds the header and non-header cell format variations for the given column.
    fn add(&mut self, c: XLSXColumn, b: FormatBuilder) {
        self.cells.insert(c, self.build_cell(b));
        self.headers.insert(c, self.build_header(b));
    }

    /// Constructs a [Format] from a [FormatBuilder] for a non-header cell.
    pub fn build_cell(&self, b: FormatBuilder) -> Format {
        let mut f = &mut Format::new();
        if b.0 != 0 {
            f = f.set_bg_color(FormatColor::Custom(b.0));
        }
        if !b.1.is_empty() {
            f = f.set_num_format(b.1);
        }
        f.to_owned()
    }

    /// Constructs a [Format] from a [FormatBuilder] for a header cell.
    fn build_header(&self, b: FormatBuilder) -> Format {
        let mut f = &mut Format::new();
        if b.0 != 0 {
            f = f.set_bg_color(FormatColor::Custom(next_color(b.0).unwrap()));
        }
        f = f.set_bold();
        f = f.set_border_bottom(FormatBorder::Thin);
        f.to_owned()
    }
}
