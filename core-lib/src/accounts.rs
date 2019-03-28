use chrono::prelude::*;
use std::collections::HashMap;

pub struct Accounts(HashMap<u32, Account>);

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Account {
    id: u32,
    name: String,
    date_created: DateTime<Local>,
}

impl Accounts {
    // init accounts, read the existing one, or create a new one
    // pub fn init() -> Self {
        
    // }
    
    // Add new account
    pub fn add_new_account(&mut self, id: u32, name: String) {
        self.0.insert(
            id,
            Account {
                id,
                name,
                date_created: Local::now(),
            },
        );
    }

    // Get accounts as a hashmap
    pub fn get_accounts(&self) -> &HashMap<u32, Account> {
        &self.0
    }

    // Save accounts
    pub fn save(&self) {} // TODO Implement!

    // Get accounts by id
    pub fn get_account_by_id(&self, id: u32) -> &Account {
        if !self.0.contains_key(&id) {
            panic!("Key: {} was not found in accounts", id);
        }
        &self.0[&id]
    }
}
