
use std::path::Path;

use crate::repo::*;

pub fn rm_command(repo :&mut Repository, program_name : String, config_name : Option<String>, file : Option<Vec<String>>) {
    // make sure the file was given and all 
    let rr = repo.root.clone();
    if let Some(v) = file.clone() {
        let existant_files = v.iter()
             // filter files that dont exist
            .filter(|fp| {
                let string_path_to_repo = format!("{}programs/{}/{}/{}", rr, &program_name, &config_name.clone().unwrap(), &fp);
                if Path::new(&(string_path_to_repo)).exists() {return true} else {println!("could not find file {}", fp); return false}});

        for f in existant_files {
            println!("removing file: {}", f.clone());
            repo.rm_file(program_name.clone(), config_name.clone().unwrap(), f.clone());
        }

        return;
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


