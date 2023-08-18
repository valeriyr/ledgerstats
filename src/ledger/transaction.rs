use thiserror::Error;

/// Type alias for timestamp.
type Timestamp = u64;
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

        Ok(Transaction {
            left: fields[0].parse::<TxId>()?,
            right: fields[1].parse::<TxId>()?,
            timestamp: fields[2].parse::<Timestamp>()?,
        })
    }
}
