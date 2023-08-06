#![feature(cursor_remaining, iter_advance_by)]

#[macro_use]
extern crate tracing;

use clap::{Parser, ValueEnum};
use tracing::Level;
use tracing_subscriber::{filter, prelude::*};

mod error;
mod formats;
mod utils;

use formats::Duplicacy;

#[derive(Parser, Debug)]
struct Args {
    /// Format
    #[arg(value_enum, short, long)]
    format: BackupFormat,

    /// Repository
    #[arg(short, long)]
    repository: String,

    /// Output directory
    #[arg(short, long)]
    output_dir: String,

    /// Password
    #[arg(short, long)]
    password: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum BackupFormat {
    Duplicacy,
}

fn main() -> miette::Result<()> {
    // initialize logging
    let filter = filter::Targets::new().with_target("backup_dumper", Level::TRACE);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let args = Args::parse();

    match args.format {
        BackupFormat::Duplicacy => {
            let mut duplicacy = Duplicacy::from_folder(args.repository, args.password)?;
            duplicacy.load_all()?;
            duplicacy.dump_all_files(args.output_dir)?;
        }
    }

    info!("Done!");

    Ok(())
}