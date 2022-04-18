pub mod db_utils;
pub mod models;
pub mod schema;

mod export_ledger;
mod parse_csv;

#[macro_use]
extern crate diesel;
extern crate csv;
extern crate dotenv;

fn main() {
    // export ledger data into csv
    export_ledger::export_ledger_data();

    parse_csv::handle_csv_parse();
}
