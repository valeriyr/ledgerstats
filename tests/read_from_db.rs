use ledgerstats::ledger::{self, LedgerError, ParseTxError, Transaction, Transactions};

#[test]
fn read_sample_db() {
    const DATABASE: &str = include_str!("../database.txt");

    let mut transactions = Transactions::new();

    transactions.insert(2, Transaction::new(1, 1, 0));
    transactions.insert(3, Transaction::new(1, 2, 0));
    transactions.insert(4, Transaction::new(2, 2, 1));
    transactions.insert(5, Transaction::new(3, 3, 2));
    transactions.insert(6, Transaction::new(3, 4, 3));

    assert_eq!(ledger::read_txs_from_db(DATABASE).unwrap(), transactions);
}

#[test]
fn read_db_with_empty_line_in_the_end() {
    const DATABASE: &str = "2\n\
                            1 1 0\n\
                            1 2 0\n";

    let mut transactions = Transactions::new();

    transactions.insert(2, Transaction::new(1, 1, 0));
    transactions.insert(3, Transaction::new(1, 2, 0));

    assert_eq!(ledger::read_txs_from_db(DATABASE).unwrap(), transactions);
}

#[test]
fn read_empty_db() {
    assert_eq!(
        ledger::read_txs_from_db("").unwrap_err(),
        LedgerError::EmptyDatabase
    );
}

#[test]
fn read_db_with_broken_transactions_number() {
    const DATABASE: &str = "🦀\n\
                            1 1 0\n\
                            1 2 0";

    assert!(matches!(
        ledger::read_txs_from_db(DATABASE).unwrap_err(),
        LedgerError::ParseIntError(_)
    ));
}

#[test]
fn read_db_with_broken_transaction() {
    const DATABASE: &str = "2\n\
                            1 1 0\n\
                            1 🦀 0";

    assert!(matches!(
        ledger::read_txs_from_db(DATABASE).unwrap_err(),
        LedgerError::ParseTxError(ParseTxError::ParseIntError(_))
    ));
}

#[test]
fn read_db_with_transactions_number_mismatch() {
    const DATABASE: &str = "5\n\
                            1 1 0\n\
                            1 2 0";

    assert_eq!(
        ledger::read_txs_from_db(DATABASE).unwrap_err(),
        LedgerError::WrongTxNumberError(5, 2)
    );
}
