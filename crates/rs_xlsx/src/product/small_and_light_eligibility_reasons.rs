use crate::error::XError;
#[cfg(test)]
use fake::Dummy;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, EnumIter)]
#[repr(u8)]
pub enum SmallAndLightIneligibilityReason {
    Adult = 1,
    Hazmat = 2,
    Meltable = 4,
    LargestDimension = 8,
    MiddleDimension = 16,
    SmallestDimension = 32,
    Weight = 64,
}

impl ToString for SmallAndLightIneligibilityReason {
    fn to_string(&self) -> String {
        use SmallAndLightIneligibilityReason::*;

        match self {
            Adult => "Adult product",
            Hazmat => "Hazmat product",
            Meltable => "Meltable product",
            LargestDimension => "Largest dimension is too long",
            MiddleDimension => "Middle dimension is too long",
            SmallestDimension => "Smallest dimensions is too long",
            Weight => "Weight is too high",
        }
        .to_string()
    }
}

/// SmallAndLightEligibilityReasons is a bitmask of SmallAndLightEligibilityReasons.
/// It allows us to store all the different eligibility reasons in a single byte,
/// as opposed to an array of strings.
#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct SmallAndLightEligibilityReasons(u8);

impl TryFrom<i16> for SmallAndLightEligibilityReasons {
    type Error = XError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(SmallAndLightEligibilityReasons(value as u8))
    }
}
// impl From<i16> for SmallAndLightEligibilityReasons {
//     fn from(value: i16) -> Self {
//         SmallAndLightEligibilityReasons(value as u8)
//     }
// }

impl SmallAndLightEligibilityReasons {
    pub fn add(&mut self, reason: SmallAndLightIneligibilityReason) {
        self.0 |= reason as u8;
    }

    pub fn as_i16(&self) -> i16 {
        self.0 as i16
    }

    pub fn is_ineligible(&self, reason: SmallAndLightIneligibilityReason) -> bool {
        self.0 & reason as u8 != 0
    }

    pub fn reasons(&self) -> Vec<SmallAndLightIneligibilityReason> {
        let mut reasons = vec![];

        for reason in SmallAndLightIneligibilityReason::iter() {
            if self.is_ineligible(reason) {
                reasons.push(reason);
            }
        }

        reasons
    }

    pub fn reason_strings(&self) -> Vec<String> {
        self.reasons().into_iter().map(|x| x.to_string()).collect()
    }
}

impl From<Vec<SmallAndLightIneligibilityReason>> for SmallAndLightEligibilityReasons {
    fn from(reasons: Vec<SmallAndLightIneligibilityReason>) -> Self {
        // Bitwise OR each SmallAndLightEligibilityReason
        let mut out: u8 = 0;

        for reason in reasons {
            out |= reason as u8;
        }

        SmallAndLightEligibilityReasons(out)
    }
}
