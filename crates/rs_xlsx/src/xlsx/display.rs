use crate::error::XResult;
use crate::product::small_and_light_eligibility_reasons::SmallAndLightEligibilityReasons;
use xlsxwriter::{Format, Worksheet};

pub enum Display<'a> {
    String(&'a str),
    OwnedOptString(Option<String>),
    OptString(Option<&'a String>),
    Int(i32),
    OptInt(Option<i32>),
    Float(f64),
    OptFloat(Option<f64>),
    Bool(bool),
    OptBool(Option<bool>),
    /// In cents
    Money(i32),
    OptMoney(Option<i32>),
    Link {
        url: String,
        text: String,
    },
    Empty,
    Todo,
    // /// In hundredths of a percent (ie 12.34% would be 1234)
    // Percent(i32),
}

impl<'a> Display<'a> {
    pub fn write(
        &self,
        row: u32,
        col: u16,
        sheet: &mut Worksheet,
        format: Option<&Format>,
    ) -> XResult<()> {
        let mut empty = || {
            let r: XResult<()> = Ok(sheet.write_string(row, col, "", format)?);
            r
        };

        match self {
            Display::String(s) => {
                sheet.write_string(row, col, s, format)?;
            }
            Display::OwnedOptString(s) => {
                let s = match s {
                    Some(s) => s,
                    None => return empty(),
                };
                sheet.write_string(row, col, s, format)?;
            }
            Display::OptString(s) => {
                let s = match s {
                    Some(s) => s,
                    None => return empty(),
                };
                sheet.write_string(row, col, s, format)?;
            }
            Display::Int(v) => {
                sheet.write_number(row, col, *v as f64, format)?;
            }
            Display::Float(v) => {
                sheet.write_number(row, col, *v, format)?;
            }
            Display::OptFloat(v) => {
                let v = match v {
                    Some(v) => v,
                    None => return empty(),
                };

                sheet.write_number(row, col, *v, format)?;
            }
            Display::Bool(v) => {
                sheet.write_boolean(row, col, *v, format)?;
            }
            Display::OptBool(v) => {
                let v = match v {
                    Some(v) => v,
                    None => return empty(),
                };
                sheet.write_boolean(row, col, *v, format)?;
            }
            Display::Money(v) => {
                sheet.write_number(row, col, *v as f64 / 100.0, format)?;
            }
            Display::OptMoney(v) => {
                let v = match v {
                    Some(v) => v,
                    None => return empty(),
                };
                sheet.write_number(row, col, *v as f64 / 100.0, format)?;
            }
            Display::Link { url, text } => {
                // We are purposefully _not_ using the `sheet.write_url()` method here because
                // it limits us to 65,530 links, which isn't enough for the larger files.
                //
                // To work around that, we use a formula here instead:
                // HYPERLINK("http://www.google.com","Google"
                let formula =
                    |url: &str, text: &str| format!("HYPERLINK(\"{}\",\"{}\")", url, text);
                return match (url.trim().is_empty(), text.trim().is_empty()) {
                    (true, true) => empty(),
                    (true, false) => Ok(sheet.write_string(row, col, text, format)?),
                    (false, true) => {
                        Ok(sheet.write_formula(row, col, formula(url, url).as_str(), format)?)
                    }
                    (false, false) => {
                        Ok(sheet.write_formula(row, col, formula(url, text).as_str(), format)?)
                    }
                };
            }
            Display::Empty => {}
            Display::Todo => {}
            Display::OptInt(v) => {
                let v = match v {
                    Some(v) => v,
                    None => return empty(),
                };
                sheet.write_number(row, col, *v as f64, format)?;
            }
        }

        Ok(())
    }
}

impl<'a> From<&'a str> for Display<'a> {
    fn from(s: &'a str) -> Self {
        Display::String(s)
    }
}

impl<'a> From<&'a String> for Display<'a> {
    fn from(s: &'a String) -> Self {
        Display::String(s)
    }
}

impl<'a> From<Option<String>> for Display<'a> {
    fn from(s: Option<String>) -> Self {
        Display::OwnedOptString(s)
    }
}

impl<'a> From<Option<&'a String>> for Display<'a> {
    fn from(s: Option<&'a String>) -> Self {
        Display::OptString(s)
    }
}

impl<'a> From<bool> for Display<'a> {
    fn from(v: bool) -> Self {
        Display::Bool(v)
    }
}

impl<'a> From<Option<bool>> for Display<'a> {
    fn from(v: Option<bool>) -> Self {
        Display::OptBool(v)
    }
}

impl<'a> From<Option<i32>> for Display<'a> {
    fn from(v: Option<i32>) -> Self {
        Display::OptInt(v)
    }
}

impl<'a> From<Option<f64>> for Display<'a> {
    fn from(v: Option<f64>) -> Self {
        Display::OptFloat(v)
    }
}

impl<'a> From<i32> for Display<'a> {
    fn from(v: i32) -> Self {
        Display::Int(v)
    }
}

impl<'a> From<SmallAndLightEligibilityReasons> for Display<'a> {
    fn from(value: SmallAndLightEligibilityReasons) -> Self {
        let s = value.reason_strings().join(", ");
        Display::OwnedOptString(Some(s))
    }
}
