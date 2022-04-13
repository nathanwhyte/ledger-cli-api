use std::env;
use std::process::Command;
use std::io::{self, Write};

fn main() {
    // check for new files in ledger home dir
    // add to database if new files detected
    import_ledger_data();
}

fn import_ledger_data() {
    let mut script_path = env::current_dir().unwrap();
    script_path.push("src/scripts/export-ledger-to-csv.sh");
    let output = Command::new("sh")
        .arg(script_path.into_os_string())
        .output()
        .expect("Error importing ledger data");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
