// checks if ~/.conman exists creates if not 
use serde::{Serialize, Deserialize};

use log::info;

use crate::startup;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    pub file_name : String,
    pub destination_path : String,
    pub hash : String,
}

impl ConfigFile {
    pub fn set_dst_to_relitive_path(&mut self) {
        // replace users home with ~
        info!("changing a file dst to relitive before : {}", self.destination_path);
        self.destination_path = self.destination_path.replace(&startup::get_home_dir().expect("unable to get home directory for user"), "~");   
        info!("changing a file dst to relitive after : {}", self.destination_path);
    }

    pub fn set_dst_to_hardcode_path(&mut self) {
        // replace ~ with users home dir
        info!("changing a file dst to hardcoded before : {}", self.destination_path);
        self.destination_path = self.destination_path.replace("~", &startup::get_home_dir().expect("unable to get home directory for user"));
        info!("changing a file dst to hardcoded after : {}", self.destination_path);

    }

    pub fn new(relitive_file_path : String) -> Option<Self> {
        let mut current_directory = std::env::current_dir().unwrap();

        current_directory.push(relitive_file_path.clone());

        if current_directory.exists() {
            info!("new config file {} full path: {}", current_directory.as_path().file_name()?.to_str().unwrap().to_string(), current_directory.as_path().to_str().unwrap().to_string());
            let mut new_file =ConfigFile { 
                file_name: current_directory.as_path().file_name().unwrap().to_str().unwrap().to_string(), 
                destination_path: current_directory.as_path().to_str().unwrap().to_string(),
                hash : super::vcs::get_hash_of_file(relitive_file_path),
            };

            new_file.set_dst_to_hardcode_path();

            return Some(new_file);
        } else {
            None
        }
    }

    pub fn update_hash(&mut self) {
        self.hash = super::vcs::get_hash_of_file(self.destination_path.clone());
    }
}


