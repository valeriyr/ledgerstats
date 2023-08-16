use thiserror::Error;

#[derive(Error, Debug)]
pub enum LedgerError {
    #[error("the database is empty")]
    EmptyDatabase,
    #[error("parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("parse node error: {0}")]
    ParseNodeError(#[from] super::node::ParseNodeError),
    #[error("std io error: {0}")]
    StdIoError(#[from] std::io::Error),
    #[error("wrong nodes number: expected '{0}', actual '{1}")]
    WrongNodesNumberError(usize, usize),
}
