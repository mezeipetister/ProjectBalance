// TODO:
// - CLI Accounting,
// - CSV import,
// - File read, save,
// - WEB interface,
// - Export Ledger,
// - Export log to csv,

pub mod files;
pub mod event;
pub mod ledger;
pub mod accounts;

extern crate chrono;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns a timestamp as a string
/// Now using it generating ID
pub fn get_timestamp_as_micros() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("{}", since_the_epoch.as_micros())
}
