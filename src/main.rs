use std::env;
use std::process::Command;

#[cfg(feature = "debug_flag")]
use std::io::{self, Write};

mod export_ledger;

fn main() {
    // export ledger data into csv
    export_ledger::export_ledger_data();
}
