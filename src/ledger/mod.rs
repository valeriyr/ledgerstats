mod error;
mod graph;
mod transaction;

use itertools::Itertools;

pub use self::error::LedgerError;

pub type Result<T> = std::result::Result<T, LedgerError>;

use self::graph::Graph;
use self::transaction::{Transaction, TxId};

use std::collections::{HashMap, VecDeque};

type Depth = usize;
type Depths = HashMap<TxId, Depth>;
type Transactions = HashMap<TxId, Transaction>;

pub struct Ledger {
    transactions: Transactions,
    graph: Graph,
    depths: Depths,
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
        writeln!(f, "Graph:")?;
        writeln!(f, "{:?}", self.graph)?;

        // Print depths debug information
        writeln!(f)?;
        writeln!(f, "Depths:")?;

        for (i, depth) in self.depths.iter().sorted_by_key(|(i, _)| *i) {
            writeln!(f, "{} - {}", i, depth)?;
        }

        write!(f, "--------------------------------")
    }
}

impl Ledger {
    pub fn avg_dag_depth(&self) -> f32 {
        self.depths.values().sum::<Depth>() as f32 / self.graph.size() as f32
    }

    pub fn avg_txs_per_depth(&self) -> f32 {
        let max_depth = self
            .depths
            .values()
            .max()
            .expect("the depths collection can not be empty");

        let mut unique_depths_counter: Vec<Depth> = vec![0; max_depth + 1];

        self.depths
            .values()
            .filter(|d| **d != 0)
            .for_each(|d| unique_depths_counter[*d] += 1);

        unique_depths_counter.iter().sum::<Depth>() as f32 / *max_depth as f32
    }

    pub fn avg_ref(&self) -> f32 {
        let mut sum = 0;

        self.graph.for_each(|_, _, e| sum += e.references);

        sum as f32 / self.graph.size() as f32
    }

    fn build(transactions: Transactions) -> Self {
        let graph = Self::build_graph(&transactions);
        let depths = Self::build_depths(&graph);

        Self {
            transactions,
            graph,
            depths,
        }
    }

    fn build_graph(transactions: &Transactions) -> Graph {
        let mut graph = Graph::new(transactions.len() + 1);

        for (id, tx) in transactions {
            graph.add_ref(*id, tx.left, tx.right);
        }

        graph
    }

    fn build_depths(graph: &Graph) -> Depths {
        let mut path = HashMap::new();
        let mut queue = VecDeque::new();

        let start = 1;

        path.insert(start, start);
        queue.push_back(start);

        while !queue.is_empty() {
            let j = queue.pop_front().expect("the queue can not be empty");

            for i in 1..=graph.size() {
                if !path.contains_key(&i) && graph.get(i, j).is_some() {
                    queue.push_back(i);
                    *path.entry(i).or_default() = j;
                }
            }
        }

        let mut depths = Depths::new();

        for i in 1..=graph.size() {
            let mut depth = 0;
            let mut j = i;

            while j != 1 {
                depth += 1;
                j = *path.get(&j).expect("the path must be valid");
            }

            *depths.entry(i).or_default() = depth;
        }

        depths
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
