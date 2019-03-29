// extern crate chrono;
extern crate core_lib;

// use chrono::prelude::*;
use core_lib::*;

fn main() {
    println!("Version: {}", core_lib::VERSION);
    
    let profiles = profile::Profiles::init();
    for p in profiles.get_profiles() {
        println!("Profile name: {}",p.get_name());
    }
    // let p = profile::Profile::new("Demo1".to_string(), "demo1".to_string());
    // p.save();
}
