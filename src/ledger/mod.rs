mod error;
mod graph;
mod transaction;

use itertools::Itertools;

pub use self::error::LedgerError;

/// Type alias for the ledger module result.
pub type Result<T> = std::result::Result<T, LedgerError>;

use self::graph::{Depth, Graph};

pub use self::graph::Transactions;
pub use self::transaction::{Timestamp, Transaction, TxId};

use std::collections::HashMap;

/// A ledger implementation.
pub struct Ledger {
    /// The list of raw transactions.
    transactions: Transactions,
    /// The transactions graph.
    graph: Graph,
}

impl std::fmt::Debug for Ledger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "------------ Ledger ------------")?;

        let transactions_number = self.transactions.len();

        // Print transactions debug information
        writeln!(f, "Transactions:")?;
        writeln!(f, "{transactions_number}")?;

        for (i, tx) in self.transactions.iter().sorted_by_key(|(i, _)| *i) {
            writeln!(f, "{} - {} {} {}", i, tx.left, tx.right, tx.timestamp)?;
        }

        // Print graph debug information
        writeln!(f)?;
        writeln!(f, "{:?}", self.graph)?;

        write!(f, "--------------------------------")
    }
}

impl Ledger {
    /// Creates a new `Ledger`instance.
    pub fn new(transactions: Transactions) -> Self {
        let graph = Graph::new(&transactions);

        Self {
            transactions,
            graph,
        }
    }

    /// Returns the average depth of the directed acyclic graph.
    pub fn avg_dag_depth(&self) -> f32 {
        self.graph.depths().values().sum::<Depth>() as f32 / self.graph.size() as f32
    }

    /// Returns the average number of transactions per depth(depth 0 is not included).
    pub fn avg_txs_per_depth(&self) -> f32 {
        let depths = self.graph.depths();

        let max_depth = depths
            .values()
            .max()
            .expect("the depths collection can not be empty");

        let mut unique_depths_counter: Vec<Depth> = vec![0; max_depth + 1];

        depths
            .values()
            .filter(|d| **d != 0)
            .for_each(|d| unique_depths_counter[*d] += 1);

        unique_depths_counter.iter().sum::<Depth>() as f32 / *max_depth as f32
    }

    /// Returns the average number of in-references per node.
    pub fn avg_ref(&self) -> f32 {
        let mut sum = 0;

        self.graph.for_each(|_, _, e| sum += e.references);

        sum as f32 / self.graph.size() as f32
    }
}

/// Reads the provided database file and creates a `Ledger` instance.
pub fn read_from_db(path: &str) -> Result<Ledger> {
    // Read the database file
    let lines = std::fs::read_to_string(path)?
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    // Basic content checking
    if lines.is_empty() {
        return Err(LedgerError::EmptyDatabase);
    }

    let mut it = lines.iter();

    let parsed_transactions_number = it
        .next()
        .expect("the lines collection must contain at least one element")
        .parse::<usize>()
        .map_err(LedgerError::ParseIntError)?;

    let transactions_number = lines.len() - 1;

    if parsed_transactions_number != transactions_number {
        return Err(LedgerError::WrongTxNumberError(
            parsed_transactions_number,
            transactions_number,
        ));
    }

    // Parse the nodes
    let transactions: Result<Vec<_>> = it
        .map(|l| l.parse::<Transaction>().map_err(LedgerError::ParseTxError))
        .collect();
    let transactions = transactions?;

    // Build a ledger instance
    let transactions: HashMap<_, _> = transactions
        .into_iter()
        .enumerate()
        .map(|(i, tx)| (i + 2, tx))
        .collect();

    Ok(Ledger::new(transactions))
}
