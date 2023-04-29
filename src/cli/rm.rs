
use crate::repo::*;

pub fn rm_command(repo : &mut Repository, program_name : String, config_name : Option<String>, file : Option<String>) {
    if file.is_some() {  
        let string_path_to_repo = repo.root.clone() + "programs/" + &program_name + "/" + &config_name.clone().unwrap() + "/" + &file.clone().unwrap();
        let file_path_in_repo = std::path::Path::new(&(string_path_to_repo));
        if !file_path_in_repo.exists() {
            println!("provided file does not exist!");
            std::process::exit(0);
        } else {
            println!("removing file: {}", file.clone().unwrap());
            repo.rm_file(program_name, config_name.unwrap(), file.unwrap());
            return;
        }
    }

    let mut program_name_found_in_repository : bool = false;
    for p in &repo.managed_programs.clone() { 
        if program_name == p.name {
            program_name_found_in_repository = true;

            if config_name.is_some() {
                let mut config_name_found_in_repository : bool = false;
                for c in &p.conifigurations { 
                    if config_name.clone().unwrap() == c.name {
                        config_name_found_in_repository = true;
                    }
                }
                if config_name_found_in_repository {
                    repo.rm_config(program_name, config_name.clone().unwrap()); 
                    println!("removing config: {}", config_name.unwrap());
                    return;
                } else {
                    println!("config {} does not exist", config_name.unwrap());
                    return;
                }
            }
        }
    }
    if program_name_found_in_repository {
        println!("removing program: {program_name}");
        repo.rm_program(program_name); 
    } else {
        println!("program {program_name} does not exist");
    }

}


