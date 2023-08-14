use crate::error::XResult;
use crate::inbound_eligibility::InboundEligibility;
use redis::aio::ConnectionManager;
use redis::{cmd, AsyncCommands};

#[allow(dead_code)]
pub async fn save_to_redis(
    mut redis: ConnectionManager,
    account_id: i64,
    marketplace: i16,
    asin: &str,
    inbound_eligibility: &InboundEligibility,
) -> XResult<()> {
    let encoded: Vec<u8> = inbound_eligibility.into();

    redis
        .hset(
            format!("ie:{}", account_id),
            format!("{}{}", marketplace, asin),
            encoded,
        )
        .await?;

    Ok(())
}

pub async fn get_from_redis(
    mut redis: ConnectionManager,
    account_id: i64,
    marketplace: i16,
    asins: &[&str],
) -> XResult<Vec<Option<InboundEligibility>>> {
    if asins.is_empty() {
        return Ok(vec![]);
    }

    let key = format!("ie:{}", account_id);
    let fields = asins
        .iter()
        .map(|x| format!("{}{}", marketplace, x))
        .collect::<Vec<_>>();

    // Not sure exactly what is happening here, but the high level response does
    // not work correctly here. So instead, we call back to the lower level cmd
    // based API.
    // let r: Vec<Option<Vec<u8>>> = redis.hget(key, fields).await?;
    let r: Vec<Option<Vec<u8>>> = cmd("HMGET")
        .arg(&key)
        .arg(&fields)
        .query_async(&mut redis)
        .await?;
    let r = r
        .into_iter()
        .map(|x| match x {
            Some(x) => InboundEligibility::try_from(x.as_ref()).map(Some),
            None => Ok(None),
        })
        .collect::<XResult<Vec<Option<InboundEligibility>>>>()?;

    Ok(r)
}
