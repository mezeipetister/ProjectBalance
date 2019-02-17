// Copyright 2019 Peter Mezei
// https://github.com/mezeipetister
// For more details please check the readme and LICENSE files enclosed.

extern crate project_balance;
use project_balance::core::*;

fn main() {
    let mut income = Account::new(9);
    let mut bank = Account::new(3811);
    let mut assets = Account::new(1);

    bank.credit_from(&mut income, 10000);
    assets.credit_from(&mut bank, 3000);

    print_account_details(income);
    print_account_details(bank);
    print_account_details(assets);
}
