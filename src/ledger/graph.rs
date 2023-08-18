use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use super::transaction::{Transaction, TxId};

/// Type alias for depth.
pub type Depth = usize;
/// Type alias for depths list.
pub type Depths = HashMap<TxId, Depth>;
/// Type alias for transactions list.
pub type Transactions = HashMap<TxId, Transaction>;

/// An adjacency matrix element type.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Element {
    /// The element's references amount.
    pub references: u32,
}

/// A transactions graph implementation.
pub struct Graph {
    /// The adjacency matrix size.
    size: usize,
    /// The adjacency matrix.
    adjacency_matrix: HashMap<TxId, HashMap<TxId, Element>>,
    /// The information about depths.
    depths: Depths,
}

impl std::fmt::Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let size = self.size();

        // Print adjacency matrix debug information
        writeln!(f, "Adjacency Matrix:")?;
        writeln!(f, "{size}")?;

        for i in 1..=size {
            for j in 1..=size {
                if let Some(element) = self.get(i, j) {
                    write!(f, "{} ", element.references)?;
                } else {
                    write!(f, "- ")?;
                }
            }

            if i != size {
                writeln!(f)?;
            }
        }

        writeln!(f)?;

        // Print depths debug information
        writeln!(f)?;
        writeln!(f, "Depths:")?;

        for (i, depth) in self.depths.iter().sorted_by_key(|(i, _)| *i) {
            if *i != size {
                writeln!(f, "{} - {}", i, depth)?;
            } else {
                write!(f, "{} - {}", i, depth)?;
            }
        }

        Ok(())
    }
}

impl Graph {
    /// Creates a new `Graph` instance.
    pub fn new(transactions: &Transactions) -> Self {
        let mut graph = Self {
            size: transactions.len() + 1,
            adjacency_matrix: HashMap::new(),
            depths: Depths::new(),
        };

        for (id, tx) in transactions {
            graph.add_ref(*id, tx.left, tx.right);
        }

        graph.calculate_depths();

        graph
    }

    /// Returns the adjacency matrix size.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Calls the provided function for each element in the adjacency matrix.
    /// The elements will be provided in random order.
    pub fn for_each<F>(&self, mut f: F)
    where
        Self: Sized,
        F: FnMut(&TxId, &TxId, &Element),
    {
        self.adjacency_matrix
            .iter()
            .for_each(|(i, map)| map.iter().for_each(|(j, e)| f(i, j, e)));
    }

    /// Returns the element of the adjacency matrix by index if it exists.
    pub fn get(&self, i: TxId, j: TxId) -> Option<&Element> {
        if !self.is_valid_index(i) || !self.is_valid_index(j) {
            None
        } else if let Some(entry) = self.adjacency_matrix.get(&i) {
            entry.get(&j)
        } else {
            None
        }
    }

    /// Returns the depths collection.
    pub fn depths(&self) -> &Depths {
        &self.depths
    }

    /// Adds the element-related references.
    fn add_ref(&mut self, id: TxId, left: TxId, right: TxId) {
        if !self.is_valid_index(id) || !self.is_valid_index(left) || !self.is_valid_index(right) {
            return;
        }

        let entry = self.adjacency_matrix.entry(id).or_insert(HashMap::new());

        entry.entry(left).or_insert(Element::default()).references += 1;
        entry.entry(right).or_insert(Element::default()).references += 1;
    }

    /// Checks if the provided index is valid.
    fn is_valid_index(&self, index: TxId) -> bool {
        index > 0 || index <= self.size()
    }

    /// Calculates the depths.
    /// The adjacency matrix must be already built.
    fn calculate_depths(&mut self) {
        let mut path = HashMap::new();
        let mut queue = VecDeque::new();

        let start = 1;

        path.insert(start, start);
        queue.push_back(start);

        while !queue.is_empty() {
            let j = queue.pop_front().expect("the queue can not be empty");

            for i in 1..=self.size() {
                if !path.contains_key(&i) && self.get(i, j).is_some() {
                    queue.push_back(i);
                    *path.entry(i).or_default() = j;
                }
            }
        }

        for i in 1..=self.size() {
            let mut depth = 0;
            let mut j = i;

            while j != 1 {
                depth += 1;
                j = *path.get(&j).expect("the path must be valid");
            }

            *self.depths.entry(i).or_default() = depth;
        }
    }
}
