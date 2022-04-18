use super::schema::*;

#[derive(Queryable)]
pub struct LedgerTransaction {
    pub id: i32,
    pub date_created: String,
    pub memo: String,
}

#[derive(Insertable)]
#[table_name = "ledger_transactions"]
pub struct NewLedgerTransaction {
    pub date_created: String,
    pub memo: String,
}

#[derive(Queryable)]
pub struct LedgerEntry {
    pub id: i32,
    pub transaction_id: i32,
    pub bucket: String,
    pub currency: Option<String>,
    pub credit: bool,
    pub amount: f64,
    pub comment: Option<String>,
}

#[derive(Insertable)]
#[table_name = "ledger_entries"]
pub struct NewLedgerEntry {
    pub transaction_id: i32,
    pub bucket: String,
    pub currency: Option<String>,
    pub credit: bool,
    pub amount: f64,
    pub comment: Option<String>,
}
