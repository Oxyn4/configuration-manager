
use std::path::Path;

use crate::repo::*;

pub fn rm_command(repo :&mut Repository, program_name : String, config_name : Option<String>, file : Option<Vec<String>>) {
    
    if repo.get_program_index(program_name.clone()).is_err() {
        println!("program {program_name} does not exist in this repository");
        return
    } else if config_name.is_none() && file.is_none() {
        println!("removing program: {program_name}");
        repo.rm_program(program_name); 
        return;
    }

    if let Some(c) = config_name.clone() {
        if repo.get_config_index(program_name.clone(), c.clone()).is_err() {
            println!("config {c} does not exist for this repository");
            return;
        } else if file.is_none() {
             repo.rm_config(program_name, c.clone()); 
             println!("removing config: {c}");
            return;
        }
    }

    // make sure the file was given and all 
    let rr = repo.root.clone();
    if let Some(v) = file {
        let existant_files = v.iter()
             // filter files that dont exist
            .filter(|fp| {
                let string_path_to_repo = format!("{}programs/{}/{}/{}", rr, &program_name, &config_name.clone().unwrap(), &fp);
                if Path::new(&(string_path_to_repo)).exists() {true} else {println!("could not find file {fp}"); false}});

        for f in existant_files {
            println!("removing file: {}", f.clone());
            repo.rm_file(program_name.clone(), config_name.clone().unwrap(), f.clone());
        }
    }
}


