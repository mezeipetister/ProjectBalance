// Copyright 2019 Peter Mezei
// https://github.com/mezeipetister
// For more details please check the readme and LICENSE files enclosed.

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate project_balance;
use project_balance::core::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Bag {
    apple: i32,
    banana: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Cart {
    bags: Vec<Bag>,
}

fn add_to_cart(cart: &mut Cart, apple: i32, banana: i32) {
    cart.bags.push(Bag { apple, banana });
}

fn main() {
    let mut income = Account::new(9);
    let mut bank = Account::new(3811);
    let mut assets = Account::new(1);

    bank.credit_from(&mut income, 10000);
    assets.credit_from(&mut bank, 3000);

    print_account_details(income);
    print_account_details(bank);
    print_account_details(assets);

    // Test section!
    // Testing: yaml serialization, file write;
    let mut cart = Cart { bags: Vec::new() };
    add_to_cart(&mut cart, 1, 2);
    add_to_cart(&mut cart, 3, 4);
    add_to_cart(&mut cart, 5, 6);
    add_to_cart(&mut cart, 7, 8);
    println!("{:?}", cart);
    let s = serde_yaml::to_string(&cart).unwrap();
    println!("{}", s);

    let path = Path::new("./log.yaml");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    // Create a path to the desired file
    let path = Path::new("./log.yaml");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    let result: Cart = serde_yaml::from_str(&s).unwrap();
    println!("Result object is: {:?}", result);
}
