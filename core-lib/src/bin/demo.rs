extern crate chrono;
extern crate core_lib;

use chrono::prelude::*;
use core_lib::{event::Event, ledger::*};

fn main() {
    let mut log = init_log();

    let e1 = Event::new("Demo".to_string(), "none".to_string(), 1, 2, 3, Local::now(), "No comment".to_string(), );

    e1.print_event();
    println!("--------------------------------------");

    log.add_event(e1);

    log.save();

    println!("Hi! The current event number is: {}", log.get_event_count());
}
