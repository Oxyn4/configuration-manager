
use std::io::Write;


use super::{config_file::ConfigFile, error};

use crate::startup::users::*;

use serde::{Serialize, Deserialize};
use log::{warn, info, error};

// ~/.conman/programs/PROGRAM_NAME/NAME
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    // pub name : String,
    // pub program_name : String,
    pub root : String,
    pub managed_files : Vec<ConfigFile>,
}

impl Config {
    pub fn new(root_init : String) -> Result<Self, error::ErrorKind> {
        info!("creating a new config with root: {}", root_init);

        let root_path = std::path::Path::new(&root_init);
        if !root_path.exists() {
            std::fs::create_dir_all(root_path).expect("failed to create new config directory"); 

            return Ok(Config {
                // name : root_path.file_name().unwrap().to_owned().into_string().unwrap(),
                // program_name : root_path.parent().unwrap().to_owned().file_name().unwrap().to_owned().into_string().unwrap(),
                root : root_init,
                managed_files : Vec::new(),
            })
        }

        let name_init = std::path::Path::new(&root_init.clone())
            .file_name().unwrap()
            .to_owned().into_string().unwrap();
       
        let _program_name_init = std::path::Path::new(&root_init.clone())
            .parent().unwrap()
            .to_owned()
            .into_os_string()
            .into_string().unwrap();

            if name_init.find('/').is_some() {
                println!("config name cannot contain / foward slashes");

                return Err(error::ErrorKind::ConfigNameContainsIllegalCharacter);
            }

        let json_result = std::fs::read(root_init.clone() + "/manifest.json");

        match json_result {
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    error!("missing manifest file for {}", name_init);
                    println!("manifest file missing for {name_init}");
                } else {
                    error!("error occured opening manifest files for  {}", name_init);
                    println!("error occured opening manifest file for {name_init}");
                }

                Err(error::ErrorKind::MissingManifest)
            },
            Ok(j) => {
                info!("reading data from manifest.json");

                let data = std::string::String::from_utf8_lossy(&j); 

                info!("got data from manifest.json returing desrialised struct \n{}", data);

                let config : Config = serde_json::from_str(&data).unwrap();

                Ok(config)
            } 
        }
    }
    
    pub fn parent_program(&self) -> String {
        return String::from(std::path::Path::new(&self.root).parent().to_owned().unwrap().to_str().unwrap());
    }

    pub fn name(&self) -> String {
        return String::from(std::path::Path::new(&self.root).file_name().unwrap().to_os_string().to_str().unwrap());

    }

    pub fn get_directory_path(&self) -> String {
        // get_home_dir().unwrap() + "/.conman/programs/" + &self.parent_program() + "/" + &self.name() + "/" 
        // get_home_dir().unwrap() + "/.conman/programs/" + &self.parent_program() + "/" + &self.name() + "/"
        self.root.clone()
    }

    pub fn get_manifest_path(&self) -> String {
        // println!("{}", self.parent_program());
        // println!("{}", self.name());
        self.parent_program() + "/" + &self.name() + "/manifest.json"
    }

    pub fn does_manifest_exist(&self) -> bool {
        let manifest_path = self.get_manifest_path();

        if std::path::Path::new(&manifest_path).exists() {
            info!("config: {} for program: {} contains a manifest file", self.name(), self.parent_program());
            return true;
        }
        
        warn!("config: {} for program: {} missing manifest conman.json", self.name(), self.parent_program());

        false
    }

    pub fn directory_exists(&self) -> bool {
        std::path::Path::new(&self.get_directory_path()).exists()
    }

    pub fn push_managed_file(&mut self, file : ConfigFile) {
        self.managed_files.push(file);
    }

    pub fn write_manifest(&self) {   
        info!("writing manifest for {}", self.name());
        let manifest = self.get_manifest_path();

        if self.does_manifest_exist() {
            std::fs::remove_file(manifest.clone()).expect("unable to delete outdated manifest");
        }

        // info!("manifest path: {}", manifest);
        // println!("man {}", manifest);

        let mut manifest_d = std::fs::File::create(manifest).expect("unable to create new manifest file");
        
        let serialised_file_for_manifest = serde_json::to_string_pretty(&self).expect("failed to serialise file data");

        manifest_d.write_all(serialised_file_for_manifest.as_bytes()).expect("failed to write to manifest");       
    }

    pub fn from_manifest(path : String) -> Option<Self> {
        if !std::path::Path::new(&path).exists() {
            return None; 
        }
        
        Some(
            serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
        )
    }

    pub fn delete(&mut self) {
        std::fs::remove_dir_all(self.get_directory_path()).unwrap();  
    }
}

