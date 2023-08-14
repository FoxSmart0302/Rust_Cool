use crate::xlsx::format::money_builder::MoneyBuilder;
use lazy_static::lazy_static;

pub const PERCENT: &str = "0.00%";
pub const NUMBER: &str = "_(* #,##0_);_(* (#,##0);_(* \"\"??_);_(@_)";

lazy_static! {
    pub static ref DOLLAR_FORMATTER: String = MoneyBuilder::new().lhs("$").zero("").build();
    pub static ref BRL_FORMATTER: String = MoneyBuilder::new().lhs("R$").zero("").build();
    pub static ref EURO_FORMATTER: String = MoneyBuilder::new().lhs("€").zero("").build();
    pub static ref POUND_FORMATTER: String = MoneyBuilder::new().lhs("£").zero("").build();
    pub static ref SEK_FORMATTER: String = MoneyBuilder::new().rhs("kr").zero("").build();
    pub static ref PLN_FORMATTER: String = MoneyBuilder::new().rhs("zł").zero("").build();
    pub static ref EGP_FORMATTER: String = MoneyBuilder::new().lhs("EGP").zero("").build();
    pub static ref TRY_FORMATTER: String = MoneyBuilder::new().lhs("TRY").zero("").build();
    pub static ref SAR_FORMATTER: String = MoneyBuilder::new().lhs("SAR").zero("").build();
    pub static ref AED_FORMATTER: String = MoneyBuilder::new().lhs("AED").zero("").build();
    pub static ref INR_FORMATTER: String = MoneyBuilder::new().lhs("₹").zero("").build();
    pub static ref SGD_FORMATTER: String = MoneyBuilder::new().lhs("S$").zero("").build();
    pub static ref JPY_FORMATTER: String = MoneyBuilder::new()
        .lhs("¥")
        .decimal_places(0)
        .zero("")
        .build();
}
