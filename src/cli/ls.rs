
use crate::repo::*;

pub fn ls(repo : &mut Repository) {
    println!("Repository - {} \n", repo.root);

    if !repo.managed_programs.is_empty() {
        for program in &repo.managed_programs {
            println!("{} | {} configurations ", program.name, program.conifigurations.len());
            
            if !program.conifigurations.is_empty() {
                for config in &program.conifigurations {
                    println!("{} - {} tracked files", config.name(), config.managed_files.len());

                        for f in &config.managed_files {
                        println!("* {} -> {}", f.file_name, f.hash);
                    }
                } 
            } else {
                println!("there are no configurations for {}", program.name);
            }
            if repo.managed_programs.len() > 1 && repo.managed_programs.last() != Some(program) {
                println!();
            }
        }
    } else {
        println!("there are no programs being managed by conman");
    }


}


