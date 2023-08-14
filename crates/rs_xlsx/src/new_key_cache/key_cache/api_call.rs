use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Serialize, Deserialize, EnumIter, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum APICall {
    ListCatalogCategories,
    GetCompetitivePricing,
    GetMyFeesEstimates,
}
