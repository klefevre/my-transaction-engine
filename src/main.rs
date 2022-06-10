use std::fs;
use std::io;
use std::path;
use std::process;

use anyhow::Result;
use clap::{Parser, ValueHint};
use log::debug;

use my_transaction_engine::prelude::*;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// CSV path
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    path: path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::builder()
        .format_module_path(false)
        .format_timestamp(None)
        .init();

    let args = Args::parse();
    debug!("args: {:?}", args);

    let input = fs::File::open(args.path)?;
    let output = io::stdout();

    if let Err(err) = try_run(input, output) {
        eprintln!("Error: {}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| eprintln!("because: {}", cause));
        process::exit(1);
    }
    Ok(())
}
