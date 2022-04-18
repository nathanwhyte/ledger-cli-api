use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;
use std::env;

use crate::{
    models::{LedgerEntry, LedgerTransaction, NewLedgerEntry, NewLedgerTransaction},
    schema::{ledger_entries, ledger_transactions},
};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

pub fn insert_ledger_transaction(
    connection: &PgConnection,
    date_created: String,
    memo: String,
) -> LedgerTransaction {
    let new_ledger_transaction = NewLedgerTransaction {
        date_created: date_created.to_string(),
        memo: memo.to_string(),
    };

    diesel::insert_into(ledger_transactions::table)
        .values(&new_ledger_transaction)
        .get_result(connection)
        .expect("error adding new ledger transaction to database")
}

pub fn insert_ledger_entry(
    connection: &PgConnection,
    transaction_id: i32,
    bucket: String,
    currency: Option<String>,
    credit: bool,
    amount: f64,
    comment: Option<String>,
) -> LedgerEntry {
    let new_ledger_entry = NewLedgerEntry {
        transaction_id,
        bucket,
        currency,
        credit,
        amount,
        comment,
    };

    diesel::insert_into(ledger_entries::table)
        .values(&new_ledger_entry)
        .get_result(connection)
        .expect("error adding new ledger entry to database")
}
