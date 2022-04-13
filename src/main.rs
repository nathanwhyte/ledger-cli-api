use std::env;
use std::process::Command;

#[cfg(feature = "debug_flag")]
use std::io::{self, Write};

fn main() {
    import_ledger_data();
}

// check for new files in ledger home dir
// add to database if new files detected
#[cfg(not(feature = "debug_flag"))]
fn import_ledger_data() {
    // get the path to script (relative to project root dir)
    let mut script_path = env::current_dir().unwrap();
    script_path.push("src/scripts/export-ledger-to-csv.sh");

    // run script
    Command::new("sh")
        .arg(script_path.into_os_string())
        .output()
        .expect("Error importing ledger data");
}

// check for new files in ledger home dir
// add to database if new files detected
// DEBUG VERSION: print output of script from stdout
#[cfg(feature = "debug_flag")]
fn import_ledger_data() {
    // get the path to script (relative to project root dir)
    let mut script_path = env::current_dir().unwrap();
    script_path.push("src/scripts/export-ledger-to-csv.sh");

    // run script
    let output = Command::new("sh")
        .arg(script_path.into_os_string())
        .output()
        .expect("Error importing ledger data");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
