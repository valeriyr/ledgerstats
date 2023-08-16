mod error;
mod graph;
mod transaction;
mod types;

pub use self::error::LedgerError;

pub type Result<T> = std::result::Result<T, LedgerError>;

use self::graph::Graph;
use self::transaction::Transaction;
use self::types::TxId;

use std::collections::HashMap;

type Transactions = HashMap<TxId, Transaction>;

pub struct Ledger {
    transactions: Transactions,
    graph: Graph,
}

impl std::fmt::Debug for Ledger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "------------ Ledger ------------")?;

        let transactions_number = self.transactions.len();

        // Print transactions debug information
        writeln!(f, "Transactions:")?;
        writeln!(f, "{transactions_number}")?;

        let mut keys = self.transactions.keys().collect::<Vec<_>>();
        keys.sort();

        for key in keys.into_iter() {
            let tx = self
                .transactions
                .get(key)
                .expect("the transaction must be presented");
            writeln!(f, "{} - {} {} {}", key, tx.left, tx.right, tx.timestamp)?;
        }

        // Print graph debug information
        writeln!(f)?;
        writeln!(f, "Graph:")?;
        writeln!(f, "{:?}", self.graph)?;

        writeln!(f, "--------------------------------")
    }
}

impl Ledger {
    pub fn avg_dag_depth(&self) -> f32 {
        0.0
    }

    pub fn avg_txs_per_depth(&self) -> f32 {
        0.0
    }

    pub fn avg_ref(&self) -> f32 {
        let mut sum = 0;

        let size = self.graph.size();
        let end = size + 1;

        for i in 1..end {
            for j in 1..end {
                if let Some(element) = self.graph.get(i, j) {
                    sum += element.references;
                }
            }
        }

        sum as f32 / size as f32
    }

    fn build(transactions: Transactions) -> Self {
        let graph = Self::build_graph(&transactions);

        Self {
            transactions,
            graph,
        }
    }

    fn build_graph(transactions: &Transactions) -> Graph {
        let mut adjacency_matrix = Graph::new(transactions.len() + 1);

        for (id, tx) in transactions {
            adjacency_matrix.add(*id, tx.left, tx.right);
        }

        adjacency_matrix
    }
}

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

    Ok(Ledger::build(transactions))
}
