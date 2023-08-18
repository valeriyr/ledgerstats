use thiserror::Error;

/// Type alias for timestamp.
pub type Timestamp = u64;
/// Type alias for transaction id.
pub type TxId = usize;

/// A transaction in-memory representation.
/// Can be parsed from the provided transactions database file.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Transaction {
    /// The transaction's left parent.
    pub left: TxId,
    /// The transaction's right parent.
    pub right: TxId,
    /// The transaction's timestamp.
    pub timestamp: Timestamp,
}

/// Contains all possible transaction deserialization errors.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ParseTxError {
    #[error("parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("wrong fields number: expected '{0}', actual '{1}")]
    WrongFieldsNumberError(usize, usize),
}

impl std::str::FromStr for Transaction {
    type Err = ParseTxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EXPECTED_FIELDS_NUMBER: usize = 3;

        let fields = s.split_whitespace().collect::<Vec<_>>();
        let fields_number = fields.len();

        if EXPECTED_FIELDS_NUMBER != fields_number {
            return Err(ParseTxError::WrongFieldsNumberError(
                EXPECTED_FIELDS_NUMBER,
                fields_number,
            ));
        }

        Ok(Transaction::new(
            fields[0].parse::<TxId>()?,
            fields[1].parse::<TxId>()?,
            fields[2].parse::<Timestamp>()?,
        ))
    }
}

impl Transaction {
    pub fn new(left: TxId, right: TxId, timestamp: Timestamp) -> Self {
        Self {
            left,
            right,
            timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn valid_deserialization() {
        assert_eq!(
            Transaction::from_str("1 2 3").unwrap(),
            Transaction {
                left: 1,
                right: 2,
                timestamp: 3
            }
        );
    }

    #[test]
    fn fields_number_less_then_expected() {
        assert_eq!(
            Transaction::from_str("1 2").unwrap_err(),
            ParseTxError::WrongFieldsNumberError(3, 2)
        );
    }

    #[test]
    fn fields_number_more_then_expected() {
        assert_eq!(
            Transaction::from_str("1 2 3 4").unwrap_err(),
            ParseTxError::WrongFieldsNumberError(3, 4)
        );
    }

    #[test]
    fn wrong_left_parent_field() {
        assert!(matches!(
            Transaction::from_str("-1 2 3"),
            Err(ParseTxError::ParseIntError(_))
        ));
    }

    #[test]
    fn wrong_right_parent_field() {
        assert!(matches!(
            Transaction::from_str("1 O 3"),
            Err(ParseTxError::ParseIntError(_))
        ));
    }

    #[test]
    fn wrong_timestamp_field() {
        assert!(matches!(
            Transaction::from_str("1 2 🦀"),
            Err(ParseTxError::ParseIntError(_))
        ));
    }
}
