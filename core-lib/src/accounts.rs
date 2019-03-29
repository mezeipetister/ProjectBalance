// Copyright (C) 2019 by Peter Mezei

use crate::files::*;
use crate::*;
use chrono::prelude::*;
use std::collections::HashMap;

use std::fs;

use std::fs::File;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Accounts {
    profile: String,
    accounts: HashMap<u32, Account>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Account {
    id: u32,
    name: String,
    date_created: DateTime<Local>,
}

impl Accounts {
    // Add new account
    pub fn add_new_account(&mut self, id: u32, name: String) {
        self.accounts.insert(
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
        &self.accounts
    }

    // Save accounts
    pub fn save(&self) {
        write_string_to_file(
            &mut create_file_from_path(
                &get_home_path()
                    .unwrap()
                    .join(".ledger") // TODO:! Move path to settings!
                    .join(&self.profile)
                    .join("accounts.yaml"),
            )
            .unwrap(),
            &serde_yaml::to_string(self).expect("Failed to convert accounts to string"),
        )
        .expect("Error while saving accounts");
    } // TODO Implement!

    // Get accounts by id
    pub fn get_account_by_id(&self, id: u32) -> &Account {
        if !self.accounts.contains_key(&id) {
            panic!("Key: {} was not found in accounts", id);
        }
        &self.accounts[&id]
    }
}

// Init ledger log!
// Read log from fs, read and parse it!
// Returns the ledger log.
// Should use once!
pub fn init_accounts(profile: String) -> Accounts {
    // Ledger log Path!
    let path = get_home_path().unwrap().join(".ledger").join(&profile);

    // Create path if it does not exist!
    fs::create_dir_all(&path).expect("Error while creating ledger core dir during init.");

    if path.join("accounts.yaml").exists() {
        let accounts: Accounts = serde_yaml::from_str(
            &read_file_to_string(
                &mut File::open(&path.join("accounts.yaml")).expect("Error opening accounts"),
            )
            .expect("Error while reading accounts"),
        )
        .expect("Error while parsing accounts");
        return accounts;
    } else {
        let accounts = Accounts {
            profile,
            accounts: HashMap::new(),
        };
        accounts.save();
        accounts
    }
}
