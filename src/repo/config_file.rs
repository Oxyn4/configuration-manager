// checks if ~/.conman exists creates if not 
use serde::{Serialize, Deserialize};

use log::info;



use crate::startup::users::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(deserialize_with = "deserialize_dst_path")]
    #[serde(serialize_with = "serialize_dst_path")]
    pub destination_path : String,
    pub hash : String,
}

fn deserialize_dst_path<'de, D>(deserializer: D) -> Result<std::string::String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s : &str = Deserialize::deserialize(deserializer)?;
    
    info!("changing a file destination to relitive before : {}", s);

    let ss = s.to_owned();
    
    let s = &ss.replace('~', &get_home_dir().expect("unable to get home directory for user"));
        
    info!("changing a file destination to relitive after : {}", s);

    Ok(s.to_string())
}

fn serialize_dst_path<S>(st: &std::string::String, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    info!("changing a file destination to hardcoded before : {}", st);
    
    let mut dst = st.clone();

    dst = dst.replace(&get_home_dir().expect("unable to get home directory for user"), "~");   
        
    info!("changing a file destination to hardcoded after : {}", st);
    
    s.serialize_str(&dst)
}

impl ConfigFile {
    pub fn new(relitive_file_path : String) -> Option<Self> {
        let mut current_directory = std::env::current_dir().unwrap();

        current_directory.push(relitive_file_path.clone());

        if current_directory.exists() {
            info!("new config file {} full path: {}", current_directory.as_path().file_name()?.to_str().unwrap().to_string(), current_directory.as_path().to_str().unwrap().to_string());
            let new_file =ConfigFile { 
                // file_name: current_directory.as_path().file_name().unwrap().to_str().unwrap().to_string(), 
                destination_path: current_directory.as_path().to_str().unwrap().to_string(),
                hash : super::vcs::get_hash_of_file(relitive_file_path),
            };

            Some(new_file)
        } else {
            None
        }
    }

    pub fn file_name(&self) -> String {
        return std::path::Path::new(&self.destination_path).file_name().unwrap().to_os_string().to_str().unwrap().to_owned();
    }

    pub fn update_hash(&mut self) {
        self.hash = super::vcs::get_hash_of_file(self.destination_path.clone());
    }
}

