use crate::*;

use chrono::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Event {
    id: String,
    name: String,
    debit: u32,
    credit: u32,
    value: u32,
    reference_number: String,
    performance_date: DateTime<Local>,
    accounting_date: DateTime<Local>,
    comment: String,
}

impl Event {
    // Get a new event to work with!
    pub fn new(
        name: String,
        reference_number: String,
        debit: u32,
        credit: u32,
        value: u32,
        performance_date: DateTime<Local>,
        comment: String,
    ) -> Self {
        Event {
            id: get_timestamp_as_micros(),
            name,
            reference_number,
            debit,
            credit,
            value,
            performance_date,
            accounting_date: Local::now(),
            comment,
        }
    }

    // Print event to console!
    pub fn print_event(&self) {
        // Closure to print
        let print = |key: String, value: &String| println!("{0: <20} | {1}", key, value);

        print("ID".to_string(), &self.id);
        print("Debit".to_string(), &self.debit.to_string());
        print("Credit".to_string(), &self.credit.to_string());
        print("Value".to_string(), &self.value.to_string());
        print(
            "Performance date".to_string(),
            &self.performance_date.to_string(),
        );
        print(
            "Accounting date".to_string(),
            &self.accounting_date.to_string(),
        );
        print("Comment".to_string(), &self.comment);
    }
}
