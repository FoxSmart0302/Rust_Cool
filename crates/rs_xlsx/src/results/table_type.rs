use crate::product::Product;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum TableType {
    Errors,
    Products,
}

impl TableType {
    pub fn allowed(&self, product: &Product) -> bool {
        match self {
            TableType::Errors => product.has_errors(),
            TableType::Products => !product.has_errors(),
        }
    }
}
