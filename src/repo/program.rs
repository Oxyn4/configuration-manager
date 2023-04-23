

use super::config::Config;
use log::{info, warn};

// 
// ~/.conman/pr
// ograms/NAME 
#[derive(Clone, PartialEq)]
pub struct Program {
    pub name : String,
    pub conifigurations : Vec<Config>,
}

impl Program {
    pub fn new(root : String) -> Self {
        info!("creating a program struct with root: {}", root.clone());
        let root_path = std::path::Path::new(&root);
        if root_path.exists() == false {
            info!("supplied root path: {} does not exist", root);
            std::fs::create_dir(root_path).expect("failed to create a new directory for new program");

            info!("program created with root path: {}", root_path.file_name().unwrap().to_owned().into_string().unwrap());
            return Program {
                name : root_path.file_name().unwrap().to_owned().into_string().unwrap(),
                conifigurations : Vec::new()
            };
        }

        info!("program already exists");

        let config_iterator = std::fs::read_dir(root.clone()).unwrap();

        let mut configurations_init = Vec::new();

        for config in config_iterator {
            info!("found configuration in program directory");

            configurations_init.push(
                Config::new(
                    config.unwrap()
                        .path()
                        .into_os_string()
                        .into_string().unwrap() + &"/".to_owned())
                    .expect("unable to create new config"));

        }

        info!("created program with name: {}", std::path::Path::new(&root.clone()).file_name().unwrap().to_owned().into_string().unwrap());

        Program { name: std::path::Path::new(&root).file_name().unwrap().to_owned().into_string().unwrap(), conifigurations: configurations_init }
    }
    
    pub fn does_manifest_exist(&self) -> bool {
        let manifest_path = crate::startup::get_home_dir().unwrap() + "/.conman/programs/" + &self.name + "/conman.json";

        if std::path::Path::new(&manifest_path).exists() {
            info!("program: {} contains a manifest file", self.name);
            return true;
        }
        
        warn!("program: {} missing manifest conman.json", self.name);

        false
    }

    pub fn get_directory_path(&self) -> String {
        String::from(crate::startup::get_home_dir().unwrap() + "/.conman/programs/" + self.name.as_str() + "/")
    }

    pub fn directory_exists(&self) -> bool {
        std::path::Path::new(&self.get_directory_path()).exists()
    }

    pub fn delete(&mut self) {
        std::fs::remove_dir_all(std::path::Path::new(&self.get_directory_path())).unwrap();
    }
}


