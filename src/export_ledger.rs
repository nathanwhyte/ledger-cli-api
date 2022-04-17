use dotenv::dotenv;
use std::env;

use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;

// handle collecting ledger files and exporting data to csv
pub fn export_ledger_data() {
  dotenv().ok();

  let mut ledger_files: Vec<String> = Vec::new();

  // loop through all files in LEDGER_HOME
  for file in fs::read_dir(Path::new(
    &env::var("LEDGER_HOME").expect("error loading LEDGER_HOME from .env"),
  ))
  .unwrap()
  {
    let file_name_os_str = file.unwrap().path();
    let file_name_str = file_name_os_str.to_str().unwrap();

    // grab supported files to be exported
    match Path::new(file_name_str.clone())
      .extension()
      .and_then(OsStr::to_str)
    {
      Some("ledger") => ledger_files.push(String::from(file_name_str)),
      Some("dat") => ledger_files.push(String::from(file_name_str)),
      _ => continue,
    }
  }

  execute_ledger_export(ledger_files);
}

// execute ledger export command, pipe output to csv file
fn execute_ledger_export(ledger_files: Vec<String>) {
  for ledger_file in ledger_files {
    // grab a copy of the file path before ledger_file is moved
    let ledger_file_path = ledger_file.clone();

    let output = Command::new("ledger")
      .arg("-f")
      .arg(ledger_file)
      .arg("csv")
      .output()
      .expect("Error exporting ledger data.");

    // build csv output file path
    let mut csv_file_path = String::from(env::current_dir().unwrap().as_os_str().to_str().unwrap());
    csv_file_path.push_str("/data/ledger-csv/");
    let ledger_file_name = String::from(ledger_file_path.rsplit_once("/").unwrap().1);
    csv_file_path.push_str(ledger_file_name.split_once(".").unwrap().0);
    csv_file_path.push_str(".csv");

    // write to csv file
    fs::write(Path::new(csv_file_path.as_str()), &output.stdout)
      .expect("Error writing to csv file.");
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  /// test that generated files aren't empty
  fn test_files_not_empty() {
    dotenv().ok();
    for file in fs::read_dir(Path::new(
      &env::var("CSV_DATA_HOME").expect("error loading CSV_DATA_HOME from .env"),
    ))
    .unwrap()
    {
      println!(
        "testing {} ... ",
        &file
          .as_ref()
          .unwrap()
          .file_name()
          .as_os_str()
          .to_str()
          .unwrap()
      );
      let file_string =
        fs::read_to_string(file.unwrap().path().as_os_str()).expect("error reading file");
      assert!(!file_string.is_empty());
    }
  }

  #[test]
  /// test that all generated have .csv file extension
  fn test_all_files_csv() {
    dotenv().ok();
    for file in fs::read_dir(Path::new(
      &env::var("CSV_DATA_HOME").expect("error loading CSV_DATA_HOME from .env"),
    ))
    .unwrap()
    {
      let file_path = file.unwrap().path();
      let file_extension = file_path.extension().unwrap().to_str().unwrap();
      assert_eq!(file_extension, "csv");
    }
  }

  // TODO add more tests
}
