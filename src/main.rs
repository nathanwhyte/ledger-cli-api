pub mod models;
pub mod schema;

mod export_ledger;

#[macro_use]
extern crate diesel;
extern crate dotenv;

fn main() {
    // export ledger data into csv
    export_ledger::export_ledger_data();
}
