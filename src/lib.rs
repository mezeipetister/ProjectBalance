#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::NewTransaction;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &SqliteConnection, title: &str, body: &str, debit: i32, credit: i32, payment: i32) -> usize {
    use schema::transactions;

    let new_transaction = NewTransaction { title, body, debit, credit, payment };

    diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .execute(conn)
        .expect("Error saving new transaction")
}