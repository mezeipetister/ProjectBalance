use super::schema::transactions;
extern crate chrono;
use chrono::prelude::*;

#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
    pub title: String,
    pub debit: i32,
    pub credit: i32,
    pub payment: i32,
    pub time_created: NaiveDateTime
}

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub title: &'a str,
    pub debit: i32,
    pub credit: i32,
    pub payment: i32,
}