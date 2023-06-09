

use super::config::Config;
use log::{info, warn};

use crate::startup::users::*;

// ~/.conman/programs/NAME 
#[derive(Clone, PartialEq)]
pub struct Program {
    pub name : String,
    pub conifigurations : Vec<Config>,
    pub active_config : isize,
}

impl Program {
    pub fn new(root : String) -> Self {
        info!("creating a program struct with root: {}", root);
        let root_path = std::path::Path::new(&root);
        if !root_path.exists() {
            info!("supplied root path: {} does not exist", root);
            std::fs::create_dir_all(root_path).expect("failed to create a new directory for new program");

            info!("program created with root path: {}", root_path.file_name().unwrap().to_owned().into_string().unwrap());
            return Program {
                name : root_path.file_name().unwrap().to_owned().into_string().unwrap(),
                conifigurations : Vec::new(),
                active_config : -1
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
                        .into_string().unwrap() + "/")
                    .expect("unable to create new config"));

        }

        info!("created program with name: {}", std::path::Path::new(&root.clone()).file_name().unwrap().to_owned().into_string().unwrap());

        let mut deployed_init : isize = -1;
        for (i, c) in configurations_init.iter().enumerate() {
            if c.deployed == true {
                deployed_init = i as isize;
            }
        }

        Program {
            name: std::path::Path::new(&root).file_name().unwrap().to_owned().into_string().unwrap(), 
            conifigurations: configurations_init.clone(),
            active_config : deployed_init,
        }
    }
    
    pub fn does_manifest_exist(&self) -> bool {
        let manifest_path = get_home_dir().unwrap() + "/.conman/programs/" + &self.name + "/conman.json";

        if std::path::Path::new(&manifest_path).exists() {
            info!("program: {} contains a manifest file", self.name);
            return true;
        }
        
        warn!("program: {} missing manifest conman.json", self.name);

        false
    }

    pub fn switch_active_config(&mut self, config_index : usize) {
        println!("config: {}", self.name);
        println!("config index: {}", config_index);
        println!("as isize: {}", config_index as isize);
        println!("active: {}", self.active_config);
        if self.active_config == config_index as isize {
            println!("{} is already the active configuration", self.conifigurations[self.active_config as usize].name());
            return;
        }

        // -1 means no active config
        if self.active_config != -1 {
            self.conifigurations[self.active_config as usize].undeploy();
        }

        println!("{}", self.active_config);
        self.active_config = config_index as isize;

        println!("{}", self.active_config);

        self.conifigurations[config_index].deploy();
    }

    pub fn get_directory_path(&self) -> String {
        get_home_dir().unwrap() + "/.conman/programs/" + self.name.as_str() + "/"
    }

    pub fn directory_exists(&self) -> bool {
        std::path::Path::new(&self.get_directory_path()).exists()
    }

    pub fn delete(&mut self) {
        std::fs::remove_dir_all(std::path::Path::new(&self.get_directory_path())).unwrap();
    }
}


