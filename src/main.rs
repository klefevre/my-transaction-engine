use std::path;

use anyhow::Result;
use clap::{Parser, ValueHint};
use log::debug;

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

    Ok(())
}
