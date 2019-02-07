// Copyright (C) 2019 by Peter Mezei

extern crate project_balance;
use project_balance::*;

fn main(){
    let items = [1,2,3,4,5];
    for item in &items {
        print("{}", item);
    }
}
