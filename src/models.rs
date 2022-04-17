#[derive(Queryable)]
pub struct LedgerTransaction {
    pub id: i32,
    pub date_created: String,
    pub memo: String,
}

use super::schema::ledger_transactions;

#[derive(Insertable)]
#[table_name = "ledger_transactions"]
pub struct NewLedgerTransaction<'a> {
    pub date_created: &'a str,
    pub memo: &'a str,
}

#[derive(Queryable)]
pub struct LedgerEntry {
    pub id: i32,
    pub transaction_id: i32,
    pub bucket: String,
    pub currency: String,
    pub credit: bool,
    pub amount: f32,
    pub comment: String,
}
