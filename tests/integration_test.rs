use my_transaction_engine::prelude::*;
use std::io;

fn run(given: &str) -> String {
    let mut buf = vec![];

    try_run(io::Cursor::new(given), &mut buf).unwrap();

    String::from_utf8(buf).unwrap()
}

// Sanity checks

#[test]
fn test_should_skip_duplicated_tx() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  1.0
deposit,    1,  1,  1.0
deposit,    1,  1,  1.0
"#;

    let expected = r#"client,available,held,total,locked
1,1.0000,0.0000,1.0000,false
"#;

    assert_eq!(run(given), expected);
}

#[test]
fn test_should_skip_tx_with_negative_amount() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  1.0
deposit,    1,  2,  -42.0
deposit,    1,  3,  3.0
withdrawal, 1,  4,  2.0
withdrawal, 1,  5,  -42.0
"#;

    let expected = r#"client,available,held,total,locked
1,2.0000,0.0000,2.0000,false
"#;

    assert_eq!(run(given), expected);
}

// Deposit

#[test]
fn test_deposit() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  1.0
deposit,    1,  2,  2.0
deposit,    1,  3,  3.0
"#;

    let expected = r#"client,available,held,total,locked
1,6.0000,0.0000,6.0000,false
"#;

    assert_eq!(run(given), expected);
}

// Withdrawal

#[test]
fn test_withdrawal() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
withdrawal, 1,  2,  1.0
"#;

    let expected = r#"client,available,held,total,locked
1,4.0000,0.0000,4.0000,false
"#;

    assert_eq!(run(given), expected);
}

#[test]
fn test_withdrawal_with_unsufficient_funds() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
withdrawal, 1,  2,  42.0
"#;

    let expected = r#"client,available,held,total,locked
1,5.0000,0.0000,5.0000,false
"#;

    assert_eq!(run(given), expected);
}

// Dispute

#[test]
fn test_dispute_of_a_deposit() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
dispute,    1,  1,
"#;

    let expected = r#"client,available,held,total,locked
1,0.0000,5.0000,5.0000,false
"#;

    assert_eq!(run(given), expected);
}

#[test]
fn test_dispute_of_a_withdrawal() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
withdrawal, 1,  2,  2.5
dispute,    1,  2,
"#;

    let expected = r#"client,available,held,total,locked
1,5.0000,-2.5000,2.5000,false
"#;

    assert_eq!(run(given), expected);
}

#[test]
fn test_should_ignore_dispute_of_an_unexisting_tx() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
dispute,    1,  42,
"#;

    let expected = r#"client,available,held,total,locked
1,5.0000,0.0000,5.0000,false
"#;

    assert_eq!(run(given), expected);
}

// Resolve

#[test]
fn test_resolve_of_deposit() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
withdrawal, 1,  2,  2.5
dispute,    1,  1,
resolve,    1,  1,
"#;

    let expected = r#"client,available,held,total,locked
1,2.5000,0.0000,2.5000,false
"#;

    assert_eq!(run(given), expected);
}

#[test]
fn test_resolve_of_a_withdrawal() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
withdrawal, 1,  2,  2.5
dispute,    1,  2,
resolve,    1,  2,
"#;

    let expected = r#"client,available,held,total,locked
1,2.5000,0.0000,2.5000,false
"#;

    assert_eq!(run(given), expected);
}

#[test]
fn test_should_ignore_resolve_of_an_undisputed_tx() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
resolve,    1,  1,
"#;

    let expected = r#"client,available,held,total,locked
1,5.0000,0.0000,5.0000,false
"#;

    assert_eq!(run(given), expected);
}

#[test]
fn test_should_ignore_resolve_of_an_unexisting_tx() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
resolve,    1,  42,
"#;

    let expected = r#"client,available,held,total,locked
1,5.0000,0.0000,5.0000,false
"#;

    assert_eq!(run(given), expected);
}

// Chargeback

#[test]
fn test_chargeback_of_a_deposit() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
dispute,    1,  1,
chargeback, 1,  1,
"#;

    let expected = r#"client,available,held,total,locked
1,0.0000,0.0000,0.0000,true
"#;

    assert_eq!(run(given), expected);
}

#[test]
fn test_chargeback_of_a_withdrawal() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
deposit,    1,  2,  3.0
dispute,    1,  2,
chargeback, 1,  2,
"#;

    let expected = r#"client,available,held,total,locked
1,5.0000,0.0000,5.0000,true
"#;

    assert_eq!(run(given), expected);
}

#[test]
fn test_should_ignore_chargeback_of_an_undisputed_tx() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
chargeback, 1,  1,
"#;

    let expected = r#"client,available,held,total,locked
1,5.0000,0.0000,5.0000,false
"#;

    assert_eq!(run(given), expected);
}

#[test]
fn test_should_ignore_chargeback_of_an_unexisting_tx() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
chargeback, 1,  42,
"#;

    let expected = r#"client,available,held,total,locked
1,5.0000,0.0000,5.0000,false
"#;

    assert_eq!(run(given), expected);
}

// Parsing

#[test]
fn test_should_ignore_unknown_tx_type() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.0
foo,        1,  1, 42.0
foobar
"#;

    let expected = r#"client,available,held,total,locked
1,5.0000,0.0000,5.0000,false
"#;

    assert_eq!(run(given), expected);
}

#[test]
fn test_round_to_4_decimals() {
    let given = r#"type,client,tx,amount
deposit,    1,  1,  5.42190327521
"#;

    let expected = r#"client,available,held,total,locked
1,5.4219,0.0000,5.4219,false
"#;

    assert_eq!(run(given), expected);
}
