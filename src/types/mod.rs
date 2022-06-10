mod client;
mod record;
mod transaction;

pub const DECIMAL_COUNT: u32 = 4;

pub type ClientId = u16;
pub type TransactionId = u32;

pub use client::Client;
pub use record::InputRecord;
pub use transaction::{Transaction, TransactionKind};
