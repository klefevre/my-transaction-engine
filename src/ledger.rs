use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, bail, ensure, Result};
use log::debug;
use rust_decimal::Decimal;

use crate::types::{Client, ClientId, Transaction, TransactionId, TransactionKind};

#[derive(Default)]
pub struct Ledger {
    pub clients: HashMap<ClientId, Client>,
    transactions: HashMap<TransactionId, Transaction>,
    disputed_transactions: HashSet<TransactionId>,
}

impl Ledger {
    pub fn process_tx(&mut self, tx: Transaction) -> Result<()> {
        debug!("Processing tx: {:?}", &tx);

        // Sanity checks
        ensure!(tx.amount >= Decimal::ZERO, "Negative amount isn't allowed");
        if let Some(client) = self.clients.get(&tx.client_id) {
            ensure!(!client.locked, "Client locked due to chargeback");
        }

        match tx.kind {
            TransactionKind::Deposit => self.process_deposit(tx)?,
            TransactionKind::Withdrawal => self.process_withdrawal(tx)?,
            TransactionKind::Dispute => self.process_dispute(tx)?,
            TransactionKind::Resolve => self.process_resolve(tx)?,
            TransactionKind::Chargeback => self.process_chargeback(tx)?,
        }

        Ok(())
    }
}

// - Private

impl Ledger {
    fn process_deposit(&mut self, tx: Transaction) -> Result<()> {
        let tx_exists = self.transactions.contains_key(&tx.id);
        ensure!(!tx_exists, "Transaction (id={}) already processed", tx.id);

        let client = self
            .clients
            .entry(tx.client_id)
            .or_insert_with(|| Client::new(tx.client_id));

        client.available += tx.amount;
        client.total = client.available - client.held;

        self.transactions.insert(tx.id, tx);

        Ok(())
    }

    fn process_withdrawal(&mut self, tx: Transaction) -> Result<()> {
        let tx_exists = self.transactions.contains_key(&tx.id);
        ensure!(!tx_exists, "Transaction (id={}) already processed", tx.id);

        let client = self.get_mut_client(tx.client_id)?;

        let has_enough_funds = client.available - tx.amount >= Decimal::ZERO;
        ensure!(has_enough_funds, "Insufficient funds");

        client.available -= tx.amount;
        client.total = client.available - client.held;

        self.transactions.insert(tx.id, tx);

        Ok(())
    }

    fn process_dispute(&mut self, tx: Transaction) -> Result<()> {
        let disputed_tx = self.get_transaction(tx.id)?;
        let disputed_tx_kind = disputed_tx.kind;
        let disputed_tx_amount = disputed_tx.amount;

        let client = self.get_mut_client(tx.client_id)?;

        match disputed_tx_kind {
            TransactionKind::Deposit => {
                client.available -= disputed_tx_amount;
                client.held += disputed_tx_amount;
            }
            TransactionKind::Withdrawal => {
                client.available += disputed_tx_amount;
                client.held -= disputed_tx_amount;
            }
            _ => bail!("Only deposit and widthdrawal transactions can be disputed"),
        }

        self.disputed_transactions.insert(tx.id);

        Ok(())
    }

    fn process_resolve(&mut self, tx: Transaction) -> Result<()> {
        let disputed_tx = self.get_transaction(tx.id)?;
        let disputed_tx_kind = disputed_tx.kind;
        let disputed_tx_amount = disputed_tx.amount;

        let is_ref_tx_disputed = self.disputed_transactions.contains(&disputed_tx.id);
        ensure!(is_ref_tx_disputed, "Referenced transaction isn't disputed");

        let client = self.get_mut_client(tx.client_id)?;

        match disputed_tx_kind {
            TransactionKind::Deposit => {
                client.available += disputed_tx_amount;
                client.held -= disputed_tx_amount;
            }
            TransactionKind::Withdrawal => {
                client.available -= disputed_tx_amount;
                client.held += disputed_tx_amount;
            }
            _ => bail!("Only deposit and widthdrawal transactions can be resolved"),
        }

        self.disputed_transactions.remove(&tx.id);

        Ok(())
    }

    fn process_chargeback(&mut self, tx: Transaction) -> Result<()> {
        let disputed_tx = self.get_transaction(tx.id)?;
        let disputed_tx_kind = disputed_tx.kind;
        let disputed_tx_amount = disputed_tx.amount;

        let is_ref_tx_disputed = self.disputed_transactions.contains(&disputed_tx.id);
        ensure!(is_ref_tx_disputed, "Referenced transaction isn't disputed");

        let client = self.get_mut_client(tx.client_id)?;

        match disputed_tx_kind {
            TransactionKind::Deposit => {
                client.held -= disputed_tx_amount;
                client.total -= disputed_tx_amount;
                client.locked = true;
            }
            TransactionKind::Withdrawal => {
                client.held += disputed_tx_amount;
                client.total += disputed_tx_amount;
                client.locked = true;
            }
            _ => bail!("Only deposit and widthdrawal transactions can be chargedback"),
        }

        Ok(())
    }
}

// - Helpers

impl Ledger {
    fn get_mut_client(&mut self, client_id: ClientId) -> Result<&mut Client> {
        self.clients
            .get_mut(&client_id)
            .ok_or_else(|| anyhow!("Client (client_id={}) not found", client_id))
    }

    fn get_transaction(&self, tx_id: TransactionId) -> Result<&Transaction> {
        self.transactions
            .get(&tx_id)
            .ok_or_else(|| anyhow!("Transaction (tx_id={}) not found", tx_id))
    }
}
