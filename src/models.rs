#[derive(Queryable)]
pub struct LedgerTransaction {
    pub id: i32,
    pub date_created: diesel::sql_types::Date,
    pub memo: String,
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
