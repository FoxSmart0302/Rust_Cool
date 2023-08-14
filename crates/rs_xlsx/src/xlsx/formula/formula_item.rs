use crate::error::XError;
use crate::xlsx::columns::XLSXColumn;
use crate::xlsx::formula::config::Config;

pub enum FormulaItem {
    Column(XLSXColumn),
    Option(Config),
}

impl TryFrom<&str> for FormulaItem {
    type Error = XError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Weight" => Ok(FormulaItem::Column(XLSXColumn::Weight)),
            "Length" => Ok(FormulaItem::Column(XLSXColumn::Length)),
            "Width" => Ok(FormulaItem::Column(XLSXColumn::Width)),
            "Height" => Ok(FormulaItem::Column(XLSXColumn::Height)),
            "Amazon Pack Quantity" => Ok(FormulaItem::Column(XLSXColumn::AmazonPackQuantity)),
            "Supplier Pack Quantity" => {
                Ok(FormulaItem::Column(XLSXColumn::InputSupplierPackQuantity))
            }
            "Adjusted Cost" => Ok(FormulaItem::Column(XLSXColumn::InputDiscountCost)),
            "Variable Closing Fee" => Ok(FormulaItem::Column(
                XLSXColumn::AmazonFeesVariableClosingFee,
            )),
            "Per Item Fee" => Ok(FormulaItem::Column(XLSXColumn::AmazonFeesPerItemFee)),
            "Buybox Price" => Ok(FormulaItem::Column(XLSXColumn::BuyboxPrice)),
            "FBA Fees" => Ok(FormulaItem::Column(XLSXColumn::AmazonFeesFbaFees)),
            "Referral Fee" => Ok(FormulaItem::Column(XLSXColumn::AmazonFeesReferralFee)),
            "Prep Cost" => Ok(FormulaItem::Column(XLSXColumn::FinancialsPrepCost)),
            "Total COGS" => Ok(FormulaItem::Column(XLSXColumn::InputTotalCogs)),
            "Inbound Shipping" => Ok(FormulaItem::Column(XLSXColumn::FinancialsInboundShipping)),
            "Net Revenue" => Ok(FormulaItem::Column(XLSXColumn::FinancialsNetRevenue)),
            "FBA Storage Fees" => Ok(FormulaItem::Column(XLSXColumn::FinancialsFbaStorageFees)),
            "Profit" => Ok(FormulaItem::Column(XLSXColumn::FinancialsProfit)),
            "Cost" => Ok(FormulaItem::Column(XLSXColumn::InputCost)),
            "Product Discount" => Ok(FormulaItem::Column(XLSXColumn::InputDiscountPerProduct)),
            "Config::InboundShipping" => Ok(FormulaItem::Option(Config::InboundShipping)),
            "Config::SupplierDiscount" => Ok(FormulaItem::Option(Config::SupplierDiscount)),
            "Config::AmazonVat" => Ok(FormulaItem::Option(Config::AmazonVat)),
            "Config::SupplierVat" => Ok(FormulaItem::Option(Config::SupplierVat)),
            "Config::PrepSingle" => Ok(FormulaItem::Option(Config::PrepSingle)),
            "Config::PrepMultiBase" => Ok(FormulaItem::Option(Config::PrepMultiBase)),
            "Config::PrepMultiThreshold" => Ok(FormulaItem::Option(Config::PrepMultiThreshold)),
            "Config::PrepMultiAdtlCostPerUnit" => {
                Ok(FormulaItem::Option(Config::PrepMultiAdtlCostPerUnit))
            }
            _ => Err(XError::InvalidFormulaItem(value.to_string())),
        }
    }
}
