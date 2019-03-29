// Copyright (C) 2019 by Peter Mezei

use crate::event::Event;
use crate::files::*;
use crate::*;

use std::fs;

use std::fs::File;

// Store events
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LedgerLog {
    profile: String,
    events: Vec<Event>,
}

impl LedgerLog {
    pub fn save(&self) {
        write_string_to_file(
            &mut create_file_from_path(
                &get_home_path()
                    .unwrap()
                    .join(".ledger") // TODO:! Move path to settings!
                    .join(&self.profile)
                    .join("log.yaml"),
            )
            .unwrap(),
            &serde_yaml::to_string(self).expect("Failed to convert ledger log to string"),
        )
        .expect("Error while saving ledger log");
    }

    // Add event to ledger event log!
    pub fn add_event(&mut self, event: Event) {
        // Add event
        self.events.push(event);
        // Save it!
        self.save();
    }

    // Get all events from event log!
    pub fn get_all_events(&self) -> &Vec<Event> {
        &self.events
    }

    // Get event number
    pub fn get_event_count(&self) -> u32 {
        self.events.len() as u32
    }
}

// Init ledger log!
// Read log from fs, read and parse it!
// Returns the ledger log.
// Should use once!
pub fn init_log(profile: String) -> LedgerLog {
    // Ledger log Path!
    let path = get_home_path().unwrap().join(".ledger").join(&profile);

    // Create path if it does not exist!
    fs::create_dir_all(&path).expect("Error while creatign ledger core dir during init.");

    if path.join("log.yaml").exists() {
        let log: LedgerLog = serde_yaml::from_str(
            &read_file_to_string(
                &mut File::open(&path.join("log.yaml")).expect("Error opening ledger log"),
            )
            .expect("Error while reading log"),
        )
        .expect("Error while parsing log");
        return log;
    } else {
        let log = LedgerLog {
            profile,
            events: Vec::new(),
        };
        log.save();
        log
    }
}
