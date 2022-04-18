use super::db_utils;

use csv::{ReaderBuilder, StringRecord};
use dotenv::dotenv;
use std::{collections::HashSet, env, fs, path::Path};

pub fn handle_csv_parse() {
    dotenv().ok();

    // TODO parse csv files to get data
    for file in fs::read_dir(Path::new(
        &env::var("CSV_DATA_HOME").expect("error loading CSV_DATA_HOME from .env"),
    ))
    .unwrap()
    {
        // create reader for csv data from file
        let mut csv_reader = ReaderBuilder::new()
            .from_path(file.unwrap().path())
            .unwrap();

        let connection = db_utils::establish_connection();
        let mut transactions: HashSet<String> = HashSet::new();
        let mut current_transaction_id = 0;

        for csv_line in csv_reader
            .records()
            .collect::<Result<Vec<StringRecord>, csv::Error>>()
            .unwrap()
        {
            println!("{:?}", csv_line);

            // fields for NewLedgerTransaction
            let new_date_created = csv_line.get(0).unwrap().to_string();
            let new_memo = csv_line.get(2).unwrap().to_string();

            // add memo to set, check if already processed
            // if not already processed, add new transaction to database
            if transactions.insert(new_memo.clone()) {
                current_transaction_id =
                    db_utils::insert_ledger_transaction(&connection, new_date_created, new_memo).id;
            }

            // fields for NewLedgerEntry
            let bucket = csv_line.get(3).unwrap().to_string();
            let currency = match csv_line.get(4).unwrap().is_empty() {
                true => None,
                false => Some(csv_line.get(4).unwrap().to_string()),
            };
            let credit = csv_line.get(5).unwrap().parse::<f64>().unwrap() > 0.0;
            let amount = csv_line.get(5).unwrap().parse::<f64>().unwrap();
            let comment = match csv_line.get(7).unwrap().is_empty() {
                true => None,
                false => Some(csv_line.get(7).unwrap().to_string()),
            };

            db_utils::insert_ledger_entry(
                &connection,
                current_transaction_id,
                bucket,
                currency,
                credit,
                amount,
                comment,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    /// test that input from csv files matches data expected
    /// to order fill database tables
    fn test_input_matches_expected_for_models() {}

    #[test]
    /// test that input from csv is not null for columns that
    /// specify 'not null' for their value
    fn check_input_expected_not_null() {}
}
