use thiserror::Error;

use super::types::{NodeId, Timestamp};

#[derive(Debug)]
pub struct Node {
    pub left: NodeId,
    pub right: NodeId,
    pub timestamp: Timestamp,
}

#[derive(Error, Debug)]
pub enum ParseNodeError {
    #[error("parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("wrong fields number: expected '{0}', actual '{1}")]
    WrongFieldsNumberError(usize, usize),
}

impl std::str::FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EXPECTED_FIELDS_NUMBER: usize = 3;

        let fields = s.split_whitespace().collect::<Vec<_>>();
        let fields_number = fields.len();

        if EXPECTED_FIELDS_NUMBER != fields_number {
            return Err(ParseNodeError::WrongFieldsNumberError(
                EXPECTED_FIELDS_NUMBER,
                fields_number,
            ));
        }

        Ok(Node {
            left: fields[0].parse::<NodeId>()?,
            right: fields[1].parse::<NodeId>()?,
            timestamp: fields[2].parse::<Timestamp>()?,
        })
    }
}
