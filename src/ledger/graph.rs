use std::collections::HashMap;

use super::types::TxId;

#[derive(Debug, Default, Clone, Copy)]
pub struct Element {
    pub references: u32,
}

pub struct Graph {
    size: usize,
    adjacency_matrix: HashMap<TxId, HashMap<TxId, Element>>,
}

impl std::fmt::Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let size = self.size();

        writeln!(f, "{size}")?;

        let end = size + 1;

        for i in 1..end {
            for j in 1..end {
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

        Ok(())
    }
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            adjacency_matrix: HashMap::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, i: TxId, j: TxId) -> Option<&Element> {
        if !self.is_valid_index(i) || !self.is_valid_index(j) {
            None
        } else if let Some(entry) = self.adjacency_matrix.get(&i) {
            entry.get(&j)
        } else {
            None
        }
    }

    pub fn add(&mut self, id: TxId, left: TxId, right: TxId) {
        if !self.is_valid_index(id) || !self.is_valid_index(left) || !self.is_valid_index(right) {
            return;
        }

        let entry = self.adjacency_matrix.entry(id).or_insert(HashMap::new());

        entry.entry(left).or_insert(Element::default()).references += 1;
        entry.entry(right).or_insert(Element::default()).references += 1;
    }

    fn is_valid_index(&self, index: TxId) -> bool {
        index > 0 || index <= self.size()
    }
}
