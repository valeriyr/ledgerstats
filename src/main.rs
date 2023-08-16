#![deny(unsafe_code)]

mod ledger;

use anyhow::Result;

const DB_FILE_NAME: &str = "database.txt";

fn main() -> Result<()> {
    let db_file_path = format!("./{DB_FILE_NAME}");

    let ledger = ledger::read_from_db(&db_file_path)?;

    println!("{DB_FILE_NAME}");
    println!();

    println!("{ledger:?}");

    println!("------------ Stats -------------");
    println!("AVG DAG DEPTH: {}", ledger.avg_dag_depth());
    println!("AVG TXS PER DEPTH: {}", ledger.avg_txs_per_depth());
    println!("AVG REF: {}", ledger.avg_ref());
    println!("--------------------------------");

    Ok(())
}
