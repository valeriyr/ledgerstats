//! # IOTA ledgerstats
//!
//! IOTA `ledgerstats` parses a given list in memory and returns relevant statistics.
//!
//!

#![deny(unsafe_code)]

#[macro_use]
extern crate clap;

mod ledger;

use anyhow::Result;

/// A file path with a sample list of transactions.
const DEFAULT_DB_FILE_PATH: &str = "./database.txt";

fn main() -> Result<()> {
    let matches = clap_app!(ledgerstats =>
        (version: "0.1.0")
        (author: "Valerii Reutov")
        (about: "IOTA ledgerstats parses a given list in memory and returns relevant statistics")
        (@arg DB_FILE_PATH: "path to a file containing a list of transactions")
    )
    .get_matches();

    let db_file_path = matches
        .value_of("DB_FILE_PATH")
        .unwrap_or(DEFAULT_DB_FILE_PATH);

    let ledger = ledger::read_from_db(db_file_path)?;

    println!("----------- Database -----------");
    println!("{db_file_path}");
    println!("--------------------------------");
    println!();

    println!("{ledger:?}");
    println!();

    println!("------------ Stats -------------");
    println!("AVG DAG DEPTH: {}", ledger.avg_dag_depth());
    println!("AVG TXS PER DEPTH: {}", ledger.avg_txs_per_depth());
    println!("AVG REF: {}", ledger.avg_ref());
    println!("--------------------------------");

    Ok(())
}
