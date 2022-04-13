use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;

const LEDGER_HOME: &str = "/home/natew/Dropbox/Finance/";

pub fn export_ledger_data() {
    let mut ledger_files: Vec<String> = Vec::new();

    // // loop through all files in LEDGER_HOME
    for file in fs::read_dir(Path::new(&LEDGER_HOME)).unwrap() {
        let file_name_os_str = file.unwrap().path();
        let file_name_copy = file_name_os_str.to_str().unwrap();

        // grab supported files to be exported
        match Path::new(file_name_copy.clone())
            .extension()
            .and_then(OsStr::to_str)
        {
            Some("ledger") => ledger_files.push(String::from(file_name_copy)),
            Some("dat") => ledger_files.push(String::from(file_name_copy)),
            _ => continue,
        }
    }

    execute_ledger_export(ledger_files);
}

fn execute_ledger_export(ledger_files: Vec<String>) {
    for ledger_file in ledger_files {

        let ledger_file_path = ledger_file.clone();

        let output = Command::new("ledger")
            .arg("-f")
            .arg(ledger_file)
            .arg("csv")
            .output()
            .expect("Error exporting ledger data.");

        // io::stdout().write_all(&output.stdout).unwrap();

        let mut csv_file_path = String::from(env::current_dir().unwrap().as_os_str().to_str().unwrap());
        csv_file_path.push_str("/data/ledger-csv/");

        let ledger_file_name = String::from(ledger_file_path.rsplit_once("/").unwrap().1);
        csv_file_path.push_str(ledger_file_name.split_once(".").unwrap().0);
        csv_file_path.push_str(".csv");

        fs::write(Path::new(csv_file_path.as_str()), &output.stdout).expect("Error writing to csv file.");
    }
}
