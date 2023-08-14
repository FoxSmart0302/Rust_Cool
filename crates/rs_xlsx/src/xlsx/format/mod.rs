//! This module contains various formatters. These get used to change the cell's
//! background color, font weight (ie - bold), and it's number format.
//!
//! Number formats are to show the number as a currency, a percentage, or a
//! number with a certain number of decimal places.
pub mod col_visitor;
pub mod colors;
mod formatters;
mod money_builder;
pub mod numbers;

pub use formatters::Formatters;
pub use money_builder::MoneyBuilder;

pub const GREEN: u32 = colors::EMERALD_50;
pub const GRAY: u32 = colors::NEUTRAL_200;
pub const BLUE: u32 = colors::SKY_50;
pub const ORANGE: u32 = colors::AMBER_100;
pub const RED: u32 = colors::RED_100;

pub const BLUE_DARK: u32 = 0x95B3D7;
pub const BLUE_MID: u32 = 0xB8CCE4;
pub const BLUE_LIGHT: u32 = 0xDCE6F1;
