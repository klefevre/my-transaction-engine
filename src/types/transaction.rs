use rust_decimal::Decimal;
use serde::Deserialize;

use super::{ClientId, InputRecord, TransactionId};

#[derive(Debug, Deserialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug)]
pub struct Transaction {
    pub id: TransactionId,
    pub kind: TransactionKind,
    pub client_id: ClientId,
    pub amount: Decimal,
}

impl From<InputRecord> for Transaction {
    fn from(input: InputRecord) -> Self {
        Self {
            id: input.transaction_id,
            kind: input.transaction_kind,
            client_id: input.client_id,
            amount: input.amount.unwrap_or_default(),
        }
    }
}
