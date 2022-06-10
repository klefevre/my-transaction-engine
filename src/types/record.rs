use rust_decimal::prelude::*;
use serde::{de::Error, Deserialize, Deserializer};

use super::{ClientId, TransactionId, TransactionKind, DECIMAL_COUNT};

#[derive(Debug, Deserialize)]
pub struct InputRecord {
    #[serde(rename = "type")]
    pub transaction_kind: TransactionKind,
    #[serde(rename = "client")]
    pub client_id: ClientId,
    #[serde(rename = "tx")]
    pub transaction_id: TransactionId,
    #[serde(rename = "amount", deserialize_with = "de_amount")]
    pub amount: Option<Decimal>,
}

fn de_amount<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(None);
    }
    let d = Decimal::from_str(s)
        .map(|d| d.round_dp(DECIMAL_COUNT))
        .map_err(D::Error::custom)?;

    Ok(Some(d))
}
