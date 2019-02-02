#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{Transaction, NewTransaction};

// Database connection
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// Create transaction
pub fn create_transaction<'a>(conn: &PgConnection, title: &'a str, debit: i32, credit: i32, payment: i32) -> Transaction {
    use schema::transactions;
    
    let new_transaction = NewTransaction {
            title: title,
            debit: debit,
            credit: credit,
            payment: payment,
    };
    
    diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .get_result(conn)
        .expect("Error inserting new transaction.")
}