use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Entry {
    /// Entry name, need to be unique
    pub name: String,

    /// Default label to used if not found traduction
    pub label: Option<String>,

    /// Default tip to used if not found traduction
    pub tip: Option<String>,

    pub command: String,

    /// Is can only be used by folder
    pub only_folder: Option<bool>,

    /// Max folder can be used by this command
    pub max_folder: Option<u8>,

    /// If all file path are give to one command
    pub command_each_files: Option<bool>,
}
