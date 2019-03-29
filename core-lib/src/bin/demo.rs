// extern crate chrono;
extern crate core_lib;

// use chrono::prelude::*;
use core_lib::*;

fn main() {
    println!("Version: {}", core_lib::VERSION);

    let mut _profiles = profile::init_profiles();
    _profiles.create_new_profile("Alma és banán".to_string());

    for p in _profiles.get_profiles() {
        println!("Profile name: {}", p.get_name());
        let (mut _accounts, mut _logs) = p.load_profile();
        _accounts.add_new_account(1, "Immateriális javak".to_string());
        _accounts.add_new_account(2, "Készletek".to_string());
        _accounts.save();
    }
}
