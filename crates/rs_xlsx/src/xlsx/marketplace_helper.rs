use crate::xlsx::format::numbers::{
    AED_FORMATTER, BRL_FORMATTER, DOLLAR_FORMATTER, EGP_FORMATTER, EURO_FORMATTER, INR_FORMATTER,
    JPY_FORMATTER, PLN_FORMATTER, POUND_FORMATTER, SAR_FORMATTER, SEK_FORMATTER, SGD_FORMATTER,
    TRY_FORMATTER,
};
use crate::xlsx::format::MoneyBuilder;
#[cfg(test)]
use fake::Dummy;
#[cfg(test)]
use rand::Rng;
use sp_api::marketplaces::Marketplace;

#[cfg(test)]
pub struct MyFaker;

#[cfg(test)]
impl Dummy<MyFaker> for Marketplace {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &MyFaker, rng: &mut R) -> Self {
        let mid = rng.gen_range(1..=20i16);
        Marketplace::try_from(mid).unwrap()
    }
}

pub trait MarketplaceHelper {
    fn currency_formatter(&self) -> &'static str;
    fn money_builder(&self) -> MoneyBuilder;
}

// #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
// #[cfg_attr(test, derive(Dummy))]
// pub enum Marketplace {
//     Canada,
//     UnitedStates,
//     Mexico,
//     Brazil,
//     Spain,
//     UnitedKingdom,
//     France,
//     Netherlands,
//     Germany,
//     Italy,
//     Sweden,
//     Poland,
//     Egypt,
//     Turkey,
//     SaudiArabia,
//     UnitedArabEmirates,
//     India,
//     Singapore,
//     Australia,
//     Japan,
// }

impl MarketplaceHelper for Marketplace {
    fn currency_formatter(&self) -> &'static str {
        match self {
            Marketplace::Canada => &DOLLAR_FORMATTER,
            Marketplace::UnitedStates => &DOLLAR_FORMATTER,
            Marketplace::Mexico => &DOLLAR_FORMATTER,
            Marketplace::Brazil => &BRL_FORMATTER,
            Marketplace::Spain => &EURO_FORMATTER,
            Marketplace::UnitedKingdom => &POUND_FORMATTER,
            Marketplace::France => &EURO_FORMATTER,
            Marketplace::Netherlands => &EURO_FORMATTER,
            Marketplace::Germany => &EURO_FORMATTER,
            Marketplace::Italy => &EURO_FORMATTER,
            Marketplace::Sweden => &SEK_FORMATTER,
            Marketplace::Poland => &PLN_FORMATTER,
            Marketplace::Egypt => &EGP_FORMATTER,
            Marketplace::Turkey => &TRY_FORMATTER,
            Marketplace::SaudiArabia => &SAR_FORMATTER,
            Marketplace::UAE => &AED_FORMATTER,
            Marketplace::India => &INR_FORMATTER,
            Marketplace::Singapore => &SGD_FORMATTER,
            Marketplace::Australia => &DOLLAR_FORMATTER,
            Marketplace::Japan => &JPY_FORMATTER,
        }
    }

    fn money_builder(&self) -> MoneyBuilder {
        match self {
            Marketplace::Canada | Marketplace::UnitedStates | Marketplace::Mexico => {
                MoneyBuilder::new().lhs("$").zero("")
            }
            Marketplace::Brazil => MoneyBuilder::new().lhs("R$").zero(""),
            Marketplace::Spain => MoneyBuilder::new().lhs("€").zero(""),
            Marketplace::UnitedKingdom => MoneyBuilder::new().lhs("£").zero(""),
            Marketplace::France
            | Marketplace::Netherlands
            | Marketplace::Germany
            | Marketplace::Italy => MoneyBuilder::new().lhs("€").zero(""),
            Marketplace::Sweden => MoneyBuilder::new().rhs("kr").zero(""),
            Marketplace::Poland => MoneyBuilder::new().rhs("zł").zero(""),
            Marketplace::Egypt => MoneyBuilder::new().lhs("EGP").zero(""),
            Marketplace::Turkey => MoneyBuilder::new().lhs("TRY").zero(""),
            Marketplace::SaudiArabia => MoneyBuilder::new().lhs("SAR").zero(""),
            Marketplace::UAE => MoneyBuilder::new().lhs("AED").zero(""),
            Marketplace::India => MoneyBuilder::new().lhs("₹").zero(""),
            Marketplace::Singapore => MoneyBuilder::new().lhs("S$").zero(""),
            Marketplace::Australia => MoneyBuilder::new().lhs("$").zero(""),
            Marketplace::Japan => MoneyBuilder::new().lhs("¥").decimal_places(0).zero(""),
        }
    }
}
