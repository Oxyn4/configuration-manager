
use log::{info, warn};

pub fn get_home_dir() -> String {
    let home_path = std::env::var("HOME").expect("unable to get home (~) directory, from $HOME env"); 
    home_path
}
// checks if ~/.conman exists creates if not 
fn ensure_cache_exists() {
    if std::path::Path::new(&(get_home_dir() + "/.conman")).exists() {
        log::info!("existing ~/.conman/ found");
        return;
    } else {
        log::info!("creating ~/.conman/");
        std::fs::create_dir_all(get_home_dir() + "/.conman").unwrap();
    }

}

pub fn setup_fs() {
    // initialise needed directories
    ensure_cache_exists();
    
}

#[derive(Clone, PartialEq)]
struct ConfigFile {
    file_name : String,
    destination_path : String,
}

impl ConfigFile {
    pub fn new(relitive_file_path : String) -> Option<Self> {
        let mut current_directory = std::env::current_dir().unwrap();

        current_directory.push(relitive_file_path);

        if current_directory.exists() {
            Some(ConfigFile { 
                file_name: current_directory.as_path().file_name()?.to_str().unwrap().to_string(), 
                destination_path: current_directory.as_path().to_str().unwrap().to_string() 
            })
        } else {
            None
        }
    }
}

// ~/.conman/programs/PROGRAM_NAME/NAME
#[derive(Clone, PartialEq)]
struct Config {
    pub name : String,
    program_name : String,
    managed_files : Vec<ConfigFile>,
}

impl Config {
    pub fn new(program_name_init : String, name_init : String) -> Self {
        Config {
            name : name_init, 
            program_name : program_name_init,
            managed_files : Vec::new()
        }
    }

    pub fn get_directory_path(&self) -> String {
        String::from(get_home_dir() + "/.conman/programs/" + &self.program_name + "/" + &self.name + "/") 
    }

    pub fn does_manifest_exist(&self) -> bool {
        let manifest_path = get_home_dir() + "/.conman/programs/" + &self.program_name + "/" + &self.name + "/conman.json";

        if std::path::Path::new(&manifest_path).exists() {
            info!("config: {} for program: {} contains a manifest file", self.name, self.program_name);
            return true;
        }
        
        warn!("config: {} for program: {} missing manifest conman.json", self.name, self.program_name);

        false
    }

    pub fn directory_exists(&self) -> bool {
        std::path::Path::new(&self.get_directory_path()).exists()
    }
}

// ~/.conman/programs/NAME 
#[derive(Clone, PartialEq)]
struct Program {
    pub name : String,
    conifigurations : Vec<Config>,
}

impl Program {
    pub fn new(name_init : String) -> Self {
        Program { name: name_init, conifigurations: Vec::new() }
    }
    
    pub fn does_manifest_exist(&self) -> bool {
        let manifest_path = get_home_dir() + "/.conman/programs/" + &self.name + "/conman.json";

        if std::path::Path::new(&manifest_path).exists() {
            info!("progrm: {} contains a manifest file", self.name);
            return true;
        }
        
        warn!("program: {} missing manifest conman.json", self.name);

        false
    }

    pub fn get_directory_path(&self) -> String {
        String::from(get_home_dir() + "/.conman/programs/" + self.name.as_str() + "/")
    }

    pub fn directory_exists(&self) -> bool {
        std::path::Path::new(&self.get_directory_path()).exists()
    }
}

fn get_managed_programs() -> Result<Vec<Program>, ()> {
    info!("looking for programs in ~/.conman/programs/");
    let mut result : Vec<Program> = Vec::new();
    let programs_names = std::fs::read_dir(get_home_dir() + "/.conman/programs/").unwrap();
    
    for current_program_name in programs_names {
        
        let curr_result = result.len();
        
        if current_program_name.as_ref().unwrap().path().is_dir() {
            let program_name = String::from(current_program_name.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap());
            
            result.push(Program::new(program_name.clone()));
                
            result[curr_result].does_manifest_exist();

            info!("found program {} in ~/.conman/programs/ full path: {}", program_name, result[curr_result].get_directory_path()); 
            
            let configs = std::fs::read_dir(String::from(current_program_name.as_ref().unwrap().path().to_str().unwrap())).unwrap();
            for current_config in configs {
                // exlucde files only dirs
                if current_config.as_ref().unwrap().path().is_dir() {
                    let current_config_of_program = result[curr_result].conifigurations.len();

                    result[curr_result].conifigurations.push(Config::new(program_name.clone(), String::from(current_config.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap())));          
                    
                    result[curr_result].conifigurations[current_config_of_program].does_manifest_exist();

                    info!("found configuration for {} called {} full path: {}",  
                        result[curr_result].name, 
                        String::from(current_config.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap()),
                        result[curr_result].conifigurations[current_config_of_program].get_directory_path()
                    );
                }
            }
        }
    }
    Ok(result)
}

fn add_command(program : Program, config : Option<Config>, file : Option<ConfigFile>) {
    if program.directory_exists() {
        if config.is_none() && file.is_none() {
            
            println!("program: {} is already managed by conman", program.name); 
        
        } else if config.is_some() && file.is_none() {
            
            if config.as_ref().unwrap().directory_exists() {
                println!("the config directory {} already exists for {}", config.unwrap().name, program.name);
            } else {
                println!("new config to manage: {} for {}", config.as_ref().unwrap().name, program.name);
                std::fs::create_dir(std::path::Path::new(&config.unwrap().get_directory_path())).expect("Unable to create new config");
            }
        } else if config.is_some() && file.is_some() {
            if !config.as_ref().unwrap().directory_exists() {
                println!("config {} does not exist for program {}", config.unwrap().name, program.name);
            } 
            todo!("need to implement a file being added to config");
        }
    } else {
        if config.is_none() && file.is_none() {
            println!("new program to manage: {}", program.name);
            std::fs::create_dir(std::path::Path::new(&program.get_directory_path())).expect("Unable to create new program");
        } else if config.is_some() && file.is_none() || config.is_some() && file.is_some()  {
            println!("the program {} does not exist!", program.name);
        } 
    }
}

fn rm_command(program : Program, config : Option<Config>, file : Option<ConfigFile>) {
    if program.directory_exists() {
        if config.is_none() && file.is_none() {
            println!("deleting program {}", program.name);
            std::fs::remove_dir_all(program.get_directory_path()).expect("unable to remove program");
        } else if config.is_some() && file.is_none() {
            if config.as_ref().unwrap().directory_exists() {
                println!("deleting config {} for program {}", config.as_ref().unwrap().name, program.name);
                std::fs::remove_dir_all(config.as_ref().unwrap().get_directory_path()).unwrap();
            } else {
                println!("cannot delete config {} as it does not exist!", config.as_ref().unwrap().name);
            }
        } else if config.is_some() && file.is_some() {
            todo!("implement removing a file from configuration")
        }
    } else {
        println!("program does not exist!");
    }
}

fn show_usage() {
    println!("wrong usage");
    std::process::exit(1);
}

fn main() {
    std::fs::create_dir_all(get_home_dir() + "/.conman/logs/").expect("unable to create $HOME/.conman/logs/");

    simplelog::WriteLogger::init(
        simplelog::LevelFilter::max(),
        simplelog::Config::default(),
        std::fs::File::create(get_home_dir() + "/.conman/logs/conman.log").unwrap()
    ).expect("failed to intialise logger!");

    setup_fs();
    
    info!("users home path: {}", get_home_dir());

    let programs_managed_by_conman : Vec<Program> = get_managed_programs().unwrap();    
   
    let arguements : Vec<String> = std::env::args().collect();

    if arguements.len() == 1 {
        show_usage();
    }

    match arguements[1].as_str() {
        "add" => {
            // the add command adds a program, config or file 
            info!("add command called");
            
            if arguements.len() < 3 {
                show_usage();
            }

            let program = Program::new(arguements[2].to_string());

            let config : Option<Config>;

            if arguements.len() >= 4  {
                config = Some(Config::new(program.name.clone(), arguements[3].to_string())); 
            } else {
                config = None;
            }

            let file : Option<ConfigFile>;

            if arguements.len() >= 5 {
                let file_exists = ConfigFile::new(arguements[4].to_string());
                if file_exists == None {
                    println!("file {} does not exist!", arguements[4].to_string());
                    show_usage();
                }
                file = Some(file_exists.unwrap());
            } else {
                file = None; 
            }

            add_command(program, config, file);
        },
        "rm" => {
            // the add command adds a program, config or file 
            info!("rm command called");
            
            if arguements.len() < 3 {
                show_usage();
            }

            let program = Program::new(arguements[2].to_string());

            let config : Option<Config>;

            if arguements.len() >= 4  {
                config = Some(Config::new(program.name.clone(), arguements[3].to_string())); 
            } else {
                config = None;
            }

            let file : Option<ConfigFile>;

            if arguements.len() >= 5 {
                let file_exists = ConfigFile::new(arguements[4].to_string());
                if file_exists == None {
                    println!("file {} does not exist!", arguements[4].to_string());
                }
                file = Some(file_exists.unwrap());
            } else {
                file = None; 
            }

            rm_command(program, config, file);
        }
        "ls" => {
            for program in programs_managed_by_conman {
                println!(" PROGRAM | {} | {} configurations ", program.name, program.conifigurations.len());
                for config in program.conifigurations {
                    println!(" CONFIG - {}", config.name);
                }
                println!("");
            }
        },
        _ => {}
    }
}
