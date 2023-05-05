
use crate::repo::*;

pub fn add_command(repo : &mut Repository, program_name : String, config_name : Option<String>, file : Option<Vec<String>>) {
     
    if file.is_some() {
        for f in file.clone().unwrap() {
            if !std::path::Path::new(&f).exists() {
                println!("provided file does not exist!");
                std::process::exit(0);
            }
        }
    }

    let mut config_name_found_in_repository : bool = false;
    let mut program_name_found_in_repository : bool = false;
    for p in &repo.managed_programs.clone() { 
        if program_name == p.name {
            program_name_found_in_repository = true;

            if config_name.is_some() {
                for c in &p.conifigurations { 
                    if config_name.clone().unwrap() == c.name {
                        config_name_found_in_repository = true;
                    }
                }
            }
        }
    }
    if !program_name_found_in_repository {
        println!("creating program: {}", program_name.clone());
        repo.new_program(program_name.clone()); 
    }

    if !config_name_found_in_repository && config_name.is_some() {
        println!("creating config: {}", config_name.clone().unwrap());
        repo.new_config(&program_name, config_name.clone().unwrap()); 
    }

    if file.is_some() {
        for f in file.unwrap() {
            println!("adding file: {f}");
            repo.new_file(program_name.clone(), config_name.clone().unwrap(), f);
        }
    }
   
}


