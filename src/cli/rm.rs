
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
        for f in v {
            let pi = repo.get_program_index(program_name.clone()).unwrap();
            let ci = repo.get_config_index(program_name.clone(), config_name.clone().unwrap()).unwrap();
            let fi = repo.get_file_index(program_name.clone(), config_name.clone().unwrap(), f.clone());
            
            if fi.is_ok() {
                let c = &repo.managed_programs[pi].conifigurations[ci];

                let c_directory = &c.root;

                let cf_hash = &repo.managed_programs[pi].conifigurations[ci].managed_files[fi.unwrap()].hash;

                let rm_path = format!("{}/{}", c_directory, cf_hash);

                if Path::new(rm_path.as_str()).exists() {
                    println!("removing file: {}", f.clone());
                    repo.rm_file(program_name.clone(), config_name.clone().unwrap(), cf_hash.clone());
                } else {println!("file {} does not exist", f)}
            } else {println!("")}

        }
    }
}


