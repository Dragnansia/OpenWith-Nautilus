use crate::entry::Entry;
use std::{collections::HashMap, env};

pub struct Config {
    pub entries: Vec<Entry>,
}

impl Config {
    pub fn new() -> Self {
        let entries: Vec<Entry> = vec![];

        if let Ok(_lang) = env::var("LANG") {
        } else {
            // get default trad
        }

        // Get current language to load
        let _traduction: HashMap<String, String> = HashMap::new();

        // Get config file
        Self { entries }
    }
}
