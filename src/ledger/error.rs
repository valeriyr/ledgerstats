use thiserror::Error;

#[derive(Error, Debug)]
pub enum LedgerError {
    #[error("the database is empty")]
    EmptyDatabase,
    #[error("parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("parse transaction error: {0}")]
    ParseTxError(#[from] super::transaction::ParseTxError),
    #[error("std io error: {0}")]
    StdIoError(#[from] std::io::Error),
    #[error("wrong transactions number: expected '{0}', actual '{1}")]
    WrongTxNumberError(usize, usize),
}
