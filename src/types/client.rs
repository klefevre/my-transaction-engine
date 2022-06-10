use rust_decimal::Decimal;

use super::ClientId;

#[derive(Debug, Default, PartialOrd, PartialEq, Eq, Ord)]
pub struct Client {
    pub id: ClientId,
    pub locked: bool,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
}

impl Client {
    pub fn new(id: ClientId) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }
}
