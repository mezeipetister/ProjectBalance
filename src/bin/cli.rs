extern crate diesel;
extern crate project_balance;

use self::project_balance::{create_transaction, establish_connection};
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("Title?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("Debit?");
    let mut debit = String::new();
    stdin().read_line(&mut debit).unwrap();
    let debit: i32 = debit.trim().parse().expect("invalid input");
    println!("Credit?");
    let mut credit = String::new();
    stdin().read_line(&mut credit).unwrap();
    let credit: i32 = credit.trim().parse().expect("invalid input");
    println!("Payment amount?");
    let mut payment = String::new();
    stdin().read_line(&mut payment).unwrap();
    let payment: i32 = payment.trim().parse().expect("invalid input");

    let new_item = create_transaction(&connection, title, debit, credit, payment);
    println!(
        "\nSaved transaction with id {} and timestamp {}, title: {}",
        new_item.id,
        new_item.time_created,
        new_item.get_title()
    );
}
