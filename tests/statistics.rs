use ledgerstats::ledger::{Ledger, Transaction, Transactions};

#[test]
fn sample_statistics() {
    let mut transactions = Transactions::new();

    transactions.insert(2, Transaction::new(1, 1, 0));
    transactions.insert(3, Transaction::new(1, 2, 0));
    transactions.insert(4, Transaction::new(2, 2, 1));
    transactions.insert(5, Transaction::new(3, 3, 2));
    transactions.insert(6, Transaction::new(3, 4, 3));

    let ledger = Ledger::new(transactions);

    assert_eq!(ledger.avg_dag_depth(), 1.3333334);
    assert_eq!(ledger.avg_txs_per_depth(), 2.5);
    assert_eq!(ledger.avg_ref(), 1.6666666);
    assert_eq!(ledger.avg_txs_per_ts(), 1.6666666);
}

#[test]
fn empty_txs_list_statistics() {
    let transactions = Transactions::new();

    let ledger = Ledger::new(transactions);

    assert_eq!(ledger.avg_dag_depth(), 0.0);
    assert_eq!(ledger.avg_txs_per_depth(), 0.0);
    assert_eq!(ledger.avg_ref(), 0.0);
    assert_eq!(ledger.avg_txs_per_ts(), 0.0);
}

#[test]
fn one_more_graph_statistics() {
    let mut transactions = Transactions::new();

    transactions.insert(2, Transaction::new(1, 1, 0));
    transactions.insert(3, Transaction::new(1, 1, 1));
    transactions.insert(4, Transaction::new(1, 1, 2));
    transactions.insert(5, Transaction::new(2, 2, 3));

    let ledger = Ledger::new(transactions);

    assert_eq!(ledger.avg_dag_depth(), 1.0);
    assert_eq!(ledger.avg_txs_per_depth(), 2.0);
    assert_eq!(ledger.avg_ref(), 1.6);
    assert_eq!(ledger.avg_txs_per_ts(), 1.3333334);
}
