// checks if ~/.conman exists creates if not 
use serde::{Serialize, Deserialize};

use log::info;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    pub file_name : String,
    pub destination_path : String,
    pub hash : String,
}

impl ConfigFile {
    pub fn new(relitive_file_path : String) -> Option<Self> {
        let mut current_directory = std::env::current_dir().unwrap();

        current_directory.push(relitive_file_path.clone());

        if current_directory.exists() {
            info!("new config file {} full path: {}", current_directory.as_path().file_name()?.to_str().unwrap().to_string(), current_directory.as_path().to_str().unwrap().to_string());
            Some(ConfigFile { 
                file_name: current_directory.as_path().file_name().unwrap().to_str().unwrap().to_string(), 
                destination_path: current_directory.as_path().to_str().unwrap().to_string(),
                hash : super::vcs::get_hash_of_file(relitive_file_path),
            })
        } else {
            None
        }
    }

    pub fn update_hash(&mut self) {
        self.hash = super::vcs::get_hash_of_file(self.destination_path.clone());
    }
}


