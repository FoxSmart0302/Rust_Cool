mod cache;
mod codes;

use crate::error::{XError, XResult};
use crate::inbound_eligibility::codes::IneligibilityCode;
#[cfg(test)]
use fake::Dummy;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub use cache::{get_from_redis, save_to_redis};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct InboundEligibility {
    pub eligible: bool,
    pub updated_at: u32,
    pub ineligible_codes: Vec<IneligibilityCode>,
}

impl From<InboundEligibility> for Vec<u8> {
    fn from(value: InboundEligibility) -> Self {
        (&value).into()
    }
}

impl From<&InboundEligibility> for Vec<u8> {
    fn from(value: &InboundEligibility) -> Self {
        let mut out = Vec::with_capacity(5 + value.ineligible_codes.len());

        out.push(value.eligible as u8);
        out.extend_from_slice(&value.updated_at.to_le_bytes());
        for code in &value.ineligible_codes {
            out.push(u8::from(*code));
        }

        out
    }
}

impl TryFrom<&[u8]> for InboundEligibility {
    type Error = XError;

    fn try_from(value: &[u8]) -> XResult<Self> {
        let eligible = value[0] == 1;
        let updated_at = u32::from_le_bytes([value[1], value[2], value[3], value[4]]);
        let ineligible_codes = value[5..]
            .iter()
            .map(|x| IneligibilityCode::try_from(*x))
            .collect::<XResult<Vec<IneligibilityCode>>>()?;

        Ok(InboundEligibility {
            eligible,
            updated_at,
            ineligible_codes,
        })
    }
}

#[cfg(test)]
mod inbound_eligibility_test {
    use super::*;

    #[test]
    fn test_inbound_eligibility() -> XResult<()> {
        let inbound_eligibility = InboundEligibility {
            eligible: true,
            updated_at: 123,
            ineligible_codes: vec![1u8.try_into()?, 2.try_into()?, 3.try_into()?],
        };

        let bytes: Vec<u8> = inbound_eligibility.clone().into();

        let converted: InboundEligibility = bytes.as_slice().try_into()?;

        assert_eq!(inbound_eligibility, converted);
        Ok(())
    }

    #[test]
    fn it_encodes_to_json() -> XResult<()> {
        let inbound_eligibility = InboundEligibility {
            eligible: true,
            updated_at: 123,
            ineligible_codes: vec![1u8.try_into()?, 2.try_into()?, 3.try_into()?],
        };

        let json = serde_json::to_string(&inbound_eligibility).unwrap();

        assert_eq!(
            json,
            r#"{"eligible":true,"updated_at":123,"ineligible_codes":[1,2,3]}"#
        );

        Ok(())
    }
}
