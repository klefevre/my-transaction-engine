use std::collections::HashMap;
use std::io;

use anyhow::Result;
use log::debug;

use crate::ledger::Ledger;
use crate::types::{Client, ClientId, InputRecord, Transaction};

fn display_results<W: io::Write>(mut wrt: W, clients: &HashMap<ClientId, Client>) -> Result<()> {
    let mut out = "client,available,held,total,locked\n".to_string();

    for client in clients.values() {
        let line = format!(
            "{},{:.4},{:.4},{:.4},{}\n",
            client.id, client.available, client.held, client.total, client.locked
        );
        out.push_str(&line);
    }
    write!(wrt, "{}", out)?;

    Ok(())
}

fn csv_reader<R: io::Read>(rdr: R) -> Result<csv::Reader<R>> {
    let csv_rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .delimiter(b',')
        .flexible(true)
        .from_reader(rdr);

    Ok(csv_rdr)
}

pub fn try_run<R: io::Read, W: io::Write>(rdr: R, wrt: W) -> Result<()> {
    let mut ledger = Ledger::default();
    let mut csv_rdr = csv_reader(rdr)?;

    for result in csv_rdr.deserialize::<InputRecord>() {
        match result.map(Transaction::from) {
            Ok(tx) => match ledger.process_tx(tx) {
                Ok(_) => {}
                Err(e) => debug!("Can't process transaction. Reason: {}", e),
            },
            Err(e) => debug!("Can't parse row. Reason: {}", e),
        }
    }

    display_results(wrt, &ledger.clients)?;

    Ok(())
}
