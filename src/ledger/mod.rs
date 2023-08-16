mod error;
mod node;
mod types;

pub use self::error::LedgerError;

pub type Result<T> = std::result::Result<T, LedgerError>;

use std::collections::HashMap;

use self::node::Node;
use self::types::NodeId;

pub struct Ledger {
    nodes: HashMap<NodeId, Node>,
}

impl std::fmt::Debug for Ledger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "------------ Ledger ------------")?;

        // Print nodes debug information
        writeln!(f, "Nodes:")?;
        writeln!(f, "{}", self.nodes.len())?;

        let mut keys = self.nodes.keys().collect::<Vec<_>>();
        keys.sort();

        for key in keys.into_iter() {
            let node = self.nodes.get(key).expect("the node must be presented");
            writeln!(
                f,
                "{} - {} {} {}",
                key, node.left, node.right, node.timestamp
            )?;
        }

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
        0.0
    }

    fn build(nodes: HashMap<NodeId, Node>) -> Result<Self> {
        let graph = Self { nodes };

        graph.validate()?;

        Ok(graph)
    }

    fn validate(&self) -> Result<()> {
        Ok(())
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

    let expected_nodes_number = it
        .next()
        .expect("the lines collection must contain at least one element")
        .parse::<usize>()
        .map_err(LedgerError::ParseIntError)?;

    let nodes_number = lines.len() - 1;

    if expected_nodes_number != nodes_number {
        return Err(LedgerError::WrongNodesNumberError(
            expected_nodes_number,
            nodes_number,
        ));
    }

    // Parse the nodes
    let nodes: Result<Vec<_>> = it
        .map(|l| l.parse::<Node>().map_err(LedgerError::ParseNodeError))
        .collect();
    let nodes = nodes?;

    // Create a graph instance
    let nodes: HashMap<_, _> = nodes
        .into_iter()
        .enumerate()
        .map(|(i, n)| (i + 2, n))
        .collect();

    Ledger::build(nodes)
}
