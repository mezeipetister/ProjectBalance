extern crate diesel;
extern crate project_balance;

use self::project_balance::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Title?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("Body?");
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();
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

    let _ = create_post(&connection, title, &body, debit, credit, payment);
    println!("\nSaved transaction {}", title);
}