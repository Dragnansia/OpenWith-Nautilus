use crate::entry::Entry;
use std::path::Path;

#[derive(Default, Debug)]
pub struct Config {
    /// All entris to see
    pub entries: Vec<Entry>,
}

impl Config {
    pub fn new() -> Self {
        let config_path = dirs::config_dir().unwrap_or_default();
        let config_path = config_path.to_str().unwrap_or_default();
        let entries_path = format!("{config_path}/nautilus/open_with/entris.json");
        if !Path::new(&entries_path).exists() {
            return Self::default();
        }

        let content_file = std::fs::read_to_string(entries_path).unwrap_or_default();
        if let Ok(entries) = serde_json::from_str::<Vec<Entry>>(&content_file) {
            Self { entries }
        } else {
            Self::default()
        }
    }
}
