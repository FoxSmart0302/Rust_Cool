use crate::error::XError;
#[cfg(test)]
use fake::{Dummy, Faker};
#[cfg(test)]
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(into = "u8")]
#[repr(u8)]
pub enum IneligibilityCode {
    FbaInb0004,
    FbaInb0006,
    FbaInb0007,
    FbaInb0008,
    FbaInb0009,
    FbaInb0010,
    FbaInb0011,
    FbaInb0012,
    FbaInb0013,
    FbaInb0014,
    FbaInb0015,
    FbaInb0016,
    FbaInb0017,
    FbaInb0018,
    FbaInb0019,
    FbaInb0034,
    FbaInb0035,
    FbaInb0036,
    FbaInb0037,
    FbaInb0038,
    FbaInb0050,
    FbaInb0051,
    FbaInb0053,
    FbaInb0055,
    FbaInb0056,
    FbaInb0059,
    FbaInb0065,
    FbaInb0066,
    FbaInb0067,
    FbaInb0068,
    FbaInb0095,
    FbaInb0097,
    FbaInb0098,
    FbaInb0099,
    FbaInb0100,
    FbaInb0103,
    FbaInb0104,
    FbaInb0197,
    UnknownInbErrorCode,
}

#[cfg(test)]
impl Dummy<Faker> for IneligibilityCode {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let i: u8 = rng.gen_range(1..=39);
        IneligibilityCode::try_from(i).unwrap()
    }
}

impl From<IneligibilityCode> for u8 {
    fn from(value: IneligibilityCode) -> Self {
        match value {
            IneligibilityCode::FbaInb0004 => 1,
            IneligibilityCode::FbaInb0006 => 2,
            IneligibilityCode::FbaInb0007 => 3,
            IneligibilityCode::FbaInb0008 => 4,
            IneligibilityCode::FbaInb0009 => 5,
            IneligibilityCode::FbaInb0010 => 6,
            IneligibilityCode::FbaInb0011 => 7,
            IneligibilityCode::FbaInb0012 => 8,
            IneligibilityCode::FbaInb0013 => 9,
            IneligibilityCode::FbaInb0014 => 10,
            IneligibilityCode::FbaInb0015 => 11,
            IneligibilityCode::FbaInb0016 => 12,
            IneligibilityCode::FbaInb0017 => 13,
            IneligibilityCode::FbaInb0018 => 14,
            IneligibilityCode::FbaInb0019 => 15,
            IneligibilityCode::FbaInb0034 => 16,
            IneligibilityCode::FbaInb0035 => 17,
            IneligibilityCode::FbaInb0036 => 18,
            IneligibilityCode::FbaInb0037 => 19,
            IneligibilityCode::FbaInb0038 => 20,
            IneligibilityCode::FbaInb0050 => 21,
            IneligibilityCode::FbaInb0051 => 22,
            IneligibilityCode::FbaInb0053 => 23,
            IneligibilityCode::FbaInb0055 => 24,
            IneligibilityCode::FbaInb0056 => 25,
            IneligibilityCode::FbaInb0059 => 26,
            IneligibilityCode::FbaInb0065 => 27,
            IneligibilityCode::FbaInb0066 => 28,
            IneligibilityCode::FbaInb0067 => 29,
            IneligibilityCode::FbaInb0068 => 30,
            IneligibilityCode::FbaInb0095 => 31,
            IneligibilityCode::FbaInb0097 => 32,
            IneligibilityCode::FbaInb0098 => 33,
            IneligibilityCode::FbaInb0099 => 34,
            IneligibilityCode::FbaInb0100 => 35,
            IneligibilityCode::FbaInb0103 => 36,
            IneligibilityCode::FbaInb0104 => 37,
            IneligibilityCode::FbaInb0197 => 38,
            IneligibilityCode::UnknownInbErrorCode => 39,
        }
    }
}

impl TryFrom<u8> for IneligibilityCode {
    type Error = XError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(IneligibilityCode::FbaInb0004),
            2 => Ok(IneligibilityCode::FbaInb0006),
            3 => Ok(IneligibilityCode::FbaInb0007),
            4 => Ok(IneligibilityCode::FbaInb0008),
            5 => Ok(IneligibilityCode::FbaInb0009),
            6 => Ok(IneligibilityCode::FbaInb0010),
            7 => Ok(IneligibilityCode::FbaInb0011),
            8 => Ok(IneligibilityCode::FbaInb0012),
            9 => Ok(IneligibilityCode::FbaInb0013),
            10 => Ok(IneligibilityCode::FbaInb0014),
            11 => Ok(IneligibilityCode::FbaInb0015),
            12 => Ok(IneligibilityCode::FbaInb0016),
            13 => Ok(IneligibilityCode::FbaInb0017),
            14 => Ok(IneligibilityCode::FbaInb0018),
            15 => Ok(IneligibilityCode::FbaInb0019),
            16 => Ok(IneligibilityCode::FbaInb0034),
            17 => Ok(IneligibilityCode::FbaInb0035),
            18 => Ok(IneligibilityCode::FbaInb0036),
            19 => Ok(IneligibilityCode::FbaInb0037),
            20 => Ok(IneligibilityCode::FbaInb0038),
            21 => Ok(IneligibilityCode::FbaInb0050),
            22 => Ok(IneligibilityCode::FbaInb0051),
            23 => Ok(IneligibilityCode::FbaInb0053),
            24 => Ok(IneligibilityCode::FbaInb0055),
            25 => Ok(IneligibilityCode::FbaInb0056),
            26 => Ok(IneligibilityCode::FbaInb0059),
            27 => Ok(IneligibilityCode::FbaInb0065),
            28 => Ok(IneligibilityCode::FbaInb0066),
            29 => Ok(IneligibilityCode::FbaInb0067),
            30 => Ok(IneligibilityCode::FbaInb0068),
            31 => Ok(IneligibilityCode::FbaInb0095),
            32 => Ok(IneligibilityCode::FbaInb0097),
            33 => Ok(IneligibilityCode::FbaInb0098),
            34 => Ok(IneligibilityCode::FbaInb0099),
            35 => Ok(IneligibilityCode::FbaInb0100),
            36 => Ok(IneligibilityCode::FbaInb0103),
            37 => Ok(IneligibilityCode::FbaInb0104),
            38 => Ok(IneligibilityCode::FbaInb0197),
            39 => Ok(IneligibilityCode::UnknownInbErrorCode),
            _ => Err(XError::Other(format!(
                "Unknown inbound eligibility code : #{}",
                value
            ))),
        }
    }
}

impl TryFrom<&str> for IneligibilityCode {
    type Error = XError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "FBA_INB_0004" => Ok(IneligibilityCode::FbaInb0004),
            "FBA_INB_0006" => Ok(IneligibilityCode::FbaInb0006),
            "FBA_INB_0007" => Ok(IneligibilityCode::FbaInb0007),
            "FBA_INB_0008" => Ok(IneligibilityCode::FbaInb0008),
            "FBA_INB_0009" => Ok(IneligibilityCode::FbaInb0009),
            "FBA_INB_0010" => Ok(IneligibilityCode::FbaInb0010),
            "FBA_INB_0011" => Ok(IneligibilityCode::FbaInb0011),
            "FBA_INB_0012" => Ok(IneligibilityCode::FbaInb0012),
            "FBA_INB_0013" => Ok(IneligibilityCode::FbaInb0013),
            "FBA_INB_0014" => Ok(IneligibilityCode::FbaInb0014),
            "FBA_INB_0015" => Ok(IneligibilityCode::FbaInb0015),
            "FBA_INB_0016" => Ok(IneligibilityCode::FbaInb0016),
            "FBA_INB_0017" => Ok(IneligibilityCode::FbaInb0017),
            "FBA_INB_0018" => Ok(IneligibilityCode::FbaInb0018),
            "FBA_INB_0019" => Ok(IneligibilityCode::FbaInb0019),
            "FBA_INB_0034" => Ok(IneligibilityCode::FbaInb0034),
            "FBA_INB_0035" => Ok(IneligibilityCode::FbaInb0035),
            "FBA_INB_0036" => Ok(IneligibilityCode::FbaInb0036),
            "FBA_INB_0037" => Ok(IneligibilityCode::FbaInb0037),
            "FBA_INB_0038" => Ok(IneligibilityCode::FbaInb0038),
            "FBA_INB_0050" => Ok(IneligibilityCode::FbaInb0050),
            "FBA_INB_0051" => Ok(IneligibilityCode::FbaInb0051),
            "FBA_INB_0053" => Ok(IneligibilityCode::FbaInb0053),
            "FBA_INB_0055" => Ok(IneligibilityCode::FbaInb0055),
            "FBA_INB_0056" => Ok(IneligibilityCode::FbaInb0056),
            "FBA_INB_0059" => Ok(IneligibilityCode::FbaInb0059),
            "FBA_INB_0065" => Ok(IneligibilityCode::FbaInb0065),
            "FBA_INB_0066" => Ok(IneligibilityCode::FbaInb0066),
            "FBA_INB_0067" => Ok(IneligibilityCode::FbaInb0067),
            "FBA_INB_0068" => Ok(IneligibilityCode::FbaInb0068),
            "FBA_INB_0095" => Ok(IneligibilityCode::FbaInb0095),
            "FBA_INB_0097" => Ok(IneligibilityCode::FbaInb0097),
            "FBA_INB_0098" => Ok(IneligibilityCode::FbaInb0098),
            "FBA_INB_0099" => Ok(IneligibilityCode::FbaInb0099),
            "FBA_INB_0100" => Ok(IneligibilityCode::FbaInb0100),
            "FBA_INB_0103" => Ok(IneligibilityCode::FbaInb0103),
            "FBA_INB_0104" => Ok(IneligibilityCode::FbaInb0104),
            "FBA_INB_0197" => Ok(IneligibilityCode::FbaInb0197),
            "UNKNOWN_INB_ERROR_CODE" => Ok(IneligibilityCode::UnknownInbErrorCode),
            _ => Err(XError::Other(format!(
                "Unknown inbound eligibility code: {}",
                value
            ))),
        }
    }
}

impl IneligibilityCode {
    pub fn as_str(&self) -> &'static str {
        use IneligibilityCode::*;

        match self {
            FbaInb0004 =>           "Missing package dimensions. This product is missing necessary information; dimensions need to be provided in the manufacturer's original packaging.",
            FbaInb0006 =>        "The SKU for this product is unknown or cannot be found.",
            FbaInb0007 =>           "Product Under Dangerous Goods (Hazmat) Review. We do not have enough information to determine what the product is or comes with to enable us to complete our dangerous goods review. Until you provide the necessary information, the products will not be available for sale and you will not be able to send more units to Amazon fulfillment centers. You will need to add more details to the product listings, such as a clear title, bullet points, description, and image. The review process takes 4 business days.",
            FbaInb0008 =>           "Product Under Dangerous Goods (Hazmat) Review. We require detailed battery information to correctly classify the product, and until you provide the necessary information, the products will not be available for sale and you will not be able to send more units to Amazon fulfillment centers. Download an exemption sheet for battery and battery-powered products available in multiple languages in \"Upload dangerous goods documents: safety data sheet (SDS) or exemption sheet\" in Seller Central and follow instructions to submit it through the same page. The review process takes 4 business days.",
            FbaInb0009 =>           "Product Under Dangerous Goods (Hazmat) Review. We do not have enough dangerous goods information to correctly classify the product and until you provide the necessary information, the products will not be available for sale and you will not be able to send more units to Amazon fulfillment centers. Please provide a Safety Data Sheet (SDS) through \"Upload dangerous goods documents: safety data sheet (SDS) or exemption sheet\" in Seller Central, and make sure the SDS complies with all the requirements. The review process takes 4 business days.",
            FbaInb0010 =>           "Product Under Dangerous Goods (Hazmat) Review. The dangerous goods information is mismatched and so the product cannot be correctly classified. Until you provide the necessary information, the products will not be available for sale and you will not be able to send more units to Amazon fulfillment centers. Please provide compliant documents through \"Upload dangerous goods documents: safety data sheet (SDS) or exemption sheet\" in Seller Central, and make sure it complies with all the requirements. The review process takes 4 business days, the product will remain unfulfillable until review process is complete.",
            FbaInb0011 =>           "Product Under Dangerous Goods (Hazmat) Review. We have incomplete, inaccurate or conflicting dangerous goods information and cannot correctly classify the product. Until you provide the necessary information, the products will not be available for sale and you will not be able to send more units to Amazon fulfillment centers. Please provide compliant documents through \"Upload dangerous goods documents: safety data sheet (SDS) or exemption sheet\" in Seller Central, and make sure it complies with all the requirements. The review process takes 4 business days and the product will remain unfulfillable until the review process is complete.",
            FbaInb0012 =>           "Product Under Dangerous Goods (Hazmat) Review. We have determined there is conflicting product information (title, bullet points, images, or product description) within the product detail pages or with other offers for the product. Until the conflicting information is corrected, the products will not be available for sale and you will not be able to send more units to Amazon fulfillment centers. We need you to confirm the information on the product detail page The review process takes 4 business days.",
            FbaInb0013 =>           "Product Under Dangerous Goods (Hazmat) Review. Additional information is required in order to complete the Hazmat review process.",
            FbaInb0014 =>           "Product Under Dangerous Goods (Hazmat) Review. The product has been identified as possible dangerous goods. The review process generally takes 4 - 7 business days and until the review process is complete the product is unfulfillable and cannot be received at Amazon fulfilment centers or ordered by customers. For more information about dangerous goods please see \"Dangerous goods identification guide (hazmat)\" help page in Seller Central.",
            FbaInb0015 =>           "Dangerous goods (Hazmat). The product is regulated as unfulfillable and not eligible for sale with Amazon. We ask that you refrain from sending additional units in new shipments. We will need to dispose of your dangerous goods inventory in accordance with the terms of the Amazon Business Services Agreement. If you have questions or concerns, please contact Seller Support within five business days of this notice. For more information about dangerous goods please see “Dangerous goods identification guide (hazmat)” help page in Seller Central.",
            FbaInb0016 =>           "Dangerous goods (Hazmat). The product is regulated as a fulfillable dangerous good (Hazmat). You may need to be in the FBA dangerous good (Hazmat) program to be able to sell your product. For more information on the FBA dangerous good (Hazmat) program please contact Seller Support. For more information about dangerous goods please see the \"Dangerous goods identification guide (hazmat)\" help page in Seller Central.",
            FbaInb0017 =>           "This product does not exist in the destination marketplace catalog. The necessary product information will need to be provided before it can be inbounded.",
            FbaInb0018 =>           "Product missing category. This product must have a category specified before it can be sent to Amazon.",
            FbaInb0019 =>           "This product must have a title before it can be sent to Amazon.",
            FbaInb0034 =>           "Product cannot be stickerless, commingled. This product must be removed. You can send in new inventory by creating a new listing for this product that requires product labels.",
            FbaInb0035 =>           "Expiration-dated/lot-controlled product needs to be labeled. This product requires labeling to be received at our fulfillment centers.",
            FbaInb0036 =>           "Expiration-dated or lot-controlled product needs to be commingled. This product cannot be shipped to Amazon without being commingled. This error condition cannot be corrected from here. This product must be removed.",
            FbaInb0037 =>           "This product is not eligible to be shipped to our fulfillment center. You do not have all the required tax documents. If you have already filed documents please wait up to 48 hours for the data to propagate.",
            FbaInb0038 =>           "Parent ASIN cannot be fulfilled by Amazon. You can send this product by creating a listing against the child ASIN.",
            FbaInb0050 =>           "There is currently no fulfillment center in the destination country capable of receiving this product. Please delete this product from the shipment or contact Seller Support if you believe this is an error.",
            FbaInb0051 =>           "This product has been blocked by FBA and cannot currently be sent to Amazon for fulfillment.",
            FbaInb0053 =>           "Product is not eligible in the destination marketplace. This product is not eligible either because the required shipping option is not available or because the product is too large or too heavy.",
            FbaInb0055 =>           "Product unfulfillable due to media region restrictions. This product has a region code restricted for this marketplace. This product must be removed.",
            FbaInb0056 =>           "Product is ineligible for inbound. Used non-media goods cannot be shipped to Amazon.",
            FbaInb0059 =>           "Unknown Exception. This product must be removed at this time.",
            FbaInb0065 =>           "Product cannot be stickerless, commingled. This product must be removed. You can send in new inventory by creating a new listing for this product that requires product labels.",
            FbaInb0066 =>           "Unknown Exception. This product must be removed at this time.",
            FbaInb0067 =>           "Product ineligible for freight shipping. This item is ineligible for freight shipping with our Global Shipping Service. This item must be removed.",
            FbaInb0068 =>           "Account not configured for expiration-dated or lot-controlled products. Please contact TAM if you would like to configure your account to handle expiration-dated or lot-controlled inventory. Once configured, you will be able to send in this product.",
            FbaInb0095 =>           "The barcode (UPC/EAN/JAN/ISBN) for this product is associated with more than one product in our fulfillment system. This product must be removed. You can send in new inventory by creating a new listing for this product that requires product labels.",
            FbaInb0097 =>           "Fully regulated dangerous good.",
            FbaInb0098 =>           "Merchant is not authorized to send item to destination marketplace.",
            FbaInb0099 =>           "Seller account previously terminated.",
            FbaInb0100 =>           "You do not have the required tax information to send inventory to fulfillment centers in Mexico.",
            FbaInb0103 =>           "This is an expiration-dated/lot-controlled product that cannot be handled at this time.",
            FbaInb0104 =>           "Item Requires Manufacturer Barcode. Only NEW products can be stored in our fulfillment centers without product labels.",
            FbaInb0197 =>           "Item requires safety and compliance documentation. Orders for this product cannot be fulfilled by FBA without required safety and compliance documentation.",
            UnknownInbErrorCode => "Unknown Ineligibility Reason.",
        }
    }
}
