mod error;
mod graph;
mod transaction;

use itertools::Itertools;

pub use self::error::LedgerError;

/// Type alias for the ledger module result.
pub type Result<T> = std::result::Result<T, LedgerError>;

use self::graph::{Depth, Graph};

pub use self::graph::Transactions;
pub use self::transaction::{ParseTxError, Timestamp, Transaction, TxId};

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

        match max_depth {
            0 => 0.0,
            _ => depths.values().filter(|d| **d != 0).count() as f32 / *max_depth as f32,
        }
    }

    /// Returns the average number of in-references per node.
    pub fn avg_ref(&self) -> f32 {
        // TODO: is it possible to have nodes without connection to the graph?
        // (self.transactions.len() * 2) as f32 / self.graph.size() as f32

        let mut sum = 0;
        self.graph.for_each(|_, _, e| sum += e.references);

        sum as f32 / self.graph.size() as f32
    }

    /// Returns the average number of transactions per timestamp(transaction 1 is not included).
    pub fn avg_txs_per_ts(&self) -> f32 {
        if self.transactions.is_empty() {
            return 0.0;
        }

        let mut counter = HashMap::new();
        self.transactions
            .values()
            .for_each(|t| *counter.entry(t.timestamp).or_default() += 1);

        counter.values().sum::<usize>() as f32 / counter.len() as f32
    }
}

/// Reads the provided database and returns a transactions list.
pub fn read_txs_from_db(database: &str) -> Result<Transactions> {
    // Basic content checking
    let lines = database.lines().map(String::from).collect::<Vec<_>>();

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

    // Parse the transactions
    let transactions: Result<Vec<_>> = it
        .map(|l| l.parse::<Transaction>().map_err(LedgerError::ParseTxError))
        .collect();
    let transactions = transactions?;

    let transactions: HashMap<_, _> = transactions
        .into_iter()
        .enumerate()
        .map(|(i, tx)| (i + 2, tx))
        .collect();

    Ok(transactions)
}
