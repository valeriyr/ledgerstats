use thiserror::Error;

/// Contains all possible errors of the ledger module.
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum LedgerError {
    #[error("the database is empty")]
    EmptyDatabase,
    #[error("parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("parse transaction error: {0}")]
    ParseTxError(#[from] super::transaction::ParseTxError),
    #[error("wrong transactions number: expected '{0}', actual '{1}")]
    WrongTxNumberError(usize, usize),
}
