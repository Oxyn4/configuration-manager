
mod add;
use add::add_command;

mod rm;
use rm::rm_command;

mod show_usage;
use show_usage::show_usage;

use crate::repo::*;

use log::info;

pub fn handle_arguments(repo : &mut Repository) {
    
    let arguements : Vec<String> = std::env::args().collect();

    log::info!("program launched as: {:?}", arguements);
    
    if arguements.len() == 1 {
        show_usage();
        std::process::exit(1);
    }

    match arguements[1].as_str() {
        "add" | "ad" | "a" => {
            // the add command adds a program, config or file 
            info!("add command called");

            let mut program_name : String;

            let mut config_name : Option<String> = None;

            let mut file_path : Option<String> = None;

            if arguements.len() < 3 {
                show_usage();
            }

            // cm 0 add 1 neovim 2 config1 3 file 4
            program_name = arguements[2].to_owned();

            if arguements.len() > 3 {
                config_name = Some(arguements[3].to_owned());
            }

            for p in &repo.managed_programs {
                if p.to_owned().name == arguements[2] {
                    program_name = p.to_owned().name;
                } 
            
                if arguements.len() > 3 {
                    for c in &p.conifigurations {
                        if c.to_owned().name == arguements[3] {
                            config_name = Some(c.to_owned().name);
                        } 
                    }
                }
            }

            if arguements.len() > 4 {
                file_path = Some(arguements[4].to_owned());
            } else {
                file_path = None;
            }

            if program_name.find('/').is_some() {
                println!("program name cannot contain / forward slashes");
                std::process::exit(0);
            } else if config_name.is_some() && config_name.clone().unwrap().find('/').is_some() {
                println!("config name cannot contain / foward slashes");
                std::process::exit(0);
            }


            add_command(repo, program_name, config_name, file_path);
        },
        "ls" | "l"  => {
            println!("Repository - {} \n", repo.root);

            if !repo.managed_programs.is_empty() {
                for program in &repo.managed_programs {
                    println!("{} | {} configurations ", program.name, program.conifigurations.len());
                    
                    if !program.conifigurations.is_empty() {
                        for config in &program.conifigurations {
                            println!("{} - {} tracked files", config.name, config.managed_files.len());

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
        },
        "rm" | "r" => {

            let mut program_name : String;

            let mut config_name : Option<String> = None;

            let mut file_path : Option<String> = None;

            if arguements.len() < 3 {
                show_usage();
            }

            // cm 0 add 1 neovim 2 config1 3 file 4
            program_name = arguements[2].to_owned();

            if arguements.len() > 3 {
                config_name = Some(arguements[3].to_owned());
            }

            for p in &repo.managed_programs {
                if p.to_owned().name == arguements[2] {
                    program_name = p.to_owned().name;
                } 
            
                if arguements.len() > 3 {
                    for c in &p.conifigurations {
                        if c.to_owned().name == arguements[3] {
                            config_name = Some(c.to_owned().name);
                        } 
                    }
                }
            }

            if arguements.len() > 4 {
                file_path = Some(arguements[4].to_owned());
            } else {
                file_path = None;
            }

            if program_name.contains('/') {
                println!("program name cannot contain / forward slashes");
                std::process::exit(0);
            } else if config_name.is_some() && config_name.clone().unwrap().contains('/') {
                println!("config name cannot contain / foward slashes");
                std::process::exit(0);
            }

            rm_command(repo, program_name, config_name, file_path);
        },
        _ => {show_usage()}
    }

}
