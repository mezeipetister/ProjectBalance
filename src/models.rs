use super::schema::transactions;

#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub debit: i32,
    pub credit: i32,
    pub payment: i32,
    pub time_created: String
}

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub debit: i32,
    pub credit: i32,
    pub payment: i32,
}