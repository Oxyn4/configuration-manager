
use crate::repo::*;

pub fn add_command(repo : &mut Repository, program_name : String, config_name : Option<String>, file : Option<Vec<String>>) {
 
    if repo.get_program_index(program_name.clone()).is_err() {
        println!("creating program: {}", program_name.clone());
        repo.new_program(program_name.clone()); 
    } else if config_name.is_none() {
        println!("program {program_name} already managed for repository");
    }   

    if let Some(c) = &config_name {
        if repo.get_config_index(program_name.clone(), c.clone()).is_err() {
            println!("creating config: {}", c.clone());
            repo.new_config(&program_name, c.to_string());
        } else if file.is_none() {
            println!("config {c} already managed for repository");
        } 
    }

    if let Some(v) = file {
        let existant_files = v.iter().filter(|s| {
            if std::path::Path::new(s).exists() {true} else {println!("the file {s} does not exist"); false}});

        for f in existant_files {
            println!("adding file: {f}");
            repo.new_file(program_name.clone(), config_name.clone().unwrap(), f.to_string());
        }
    }
}


