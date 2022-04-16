pub mod export_ledger;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

fn main() {
  // export ledger data into csv
  export_ledger::export_ledger_data();
}
