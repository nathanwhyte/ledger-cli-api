use crate::db_utils::db_utils::establish_connection;

// use super::db_utils;

use csv::{ReaderBuilder, StringRecord};
use dotenv::dotenv;
use std::{env, fs, path::Path};

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

        // get a Vec with items being Vec with parsed csv data
        let csv_data = csv_reader
            .records()
            .collect::<Result<Vec<StringRecord>, csv::Error>>()
            .unwrap();

        for entry in csv_data {
            println!("{:?}", entry.get(0));
        }

        let _connection = establish_connection();

        // TODO construct LedgerTransaction and LedgerEntry structs (use db_utils)
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
