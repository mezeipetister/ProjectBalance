#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use chrono::prelude::*;
use schema::transactions;

#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
    pub title: String,
    pub debit: i32,
    pub credit: i32,
    pub payment: i32,
    pub time_created: NaiveDateTime,
}

impl Transaction {
    pub fn get_title(&self) -> &String {
        &self.title
    }
}

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub title: &'a str,
    pub debit: i32,
    pub credit: i32,
    pub payment: i32,
}

/// Database connection
///
/// Creating a database connection. Currently it's a Prostgres connection.
///
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// Create transaction
pub fn create_transaction<'a>(
    conn: &PgConnection,
    title: &'a str,
    debit: i32,
    credit: i32,
    payment: i32,
) -> Transaction {
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

pub fn demo_function() {
    let _a = "HelloBello".to_string();
    println!("Say: {}",_a)
}
