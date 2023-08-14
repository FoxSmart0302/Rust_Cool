use crate::error::XResult;
use crate::xlsx::coords::Coords;

pub enum Config {
    InboundShipping,
    SupplierDiscount,
    SupplierVat,
    AmazonVat,
    PrepSingle,
    PrepMultiBase,
    PrepMultiThreshold,
    PrepMultiAdtlCostPerUnit,
}

impl Config {
    pub fn coords(&self) -> &'static str {
        match self {
            Config::InboundShipping => "$C$3",
            Config::SupplierDiscount => "$C$4",
            Config::SupplierVat => "$C$5",
            Config::AmazonVat => "$C$6",
            Config::PrepSingle => "$F$3",
            Config::PrepMultiBase => "$F$4",
            Config::PrepMultiThreshold => "$F$5",
            Config::PrepMultiAdtlCostPerUnit => "$F$6",
        }
    }

    pub fn get_coords(&self) -> XResult<Coords> {
        Coords::from_a1(self.coords())
    }
}
