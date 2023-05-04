
mod add;
use add::add_command;

mod rm;
use rm::rm_command;

mod show_usage;
use show_usage::show_usage;

use crate::repo::*;

use log::info;

use clap::*;

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

fn get_cli() -> clap::Command {
    Command::new("cm") 
        .about("a tool for managing a Repository of dotfiles")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(arg!(-a --author "prints the author info"))
        .arg(arg!(-v --version "prints the program version and related info"))
        .subcommand(
            clap::Command::new("add")
                .about("add a program, config or files to the repo")
                .arg(arg!(<PROGRAM>  "the name of a program, will create a new one if does not already exist"))
                .arg(arg!([CONFIG] "the name of a config, will create a new one if does not already exist"))
                .arg(arg!([FILE] "a relitive path to a file that will be added to the specified program and config"))
        )
        .subcommand(
            clap::Command::new("rm")
                .about("remove a program, config or files to the repo")
                .arg(arg!(<PROGRAM>  "the name of a program to remove"))
                .arg(arg!([CONFIG] "the name of a config to remove from the specified program"))
                .arg(arg!([FILE] "a relitive path to a file that will be removed from the specified program and config"))
        )
        .subcommand(
            clap::Command::new("ls")
                .about("list the contents of a repository")                       
        )

    

}

fn ls(repo : &mut Repository) {
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


}

pub fn cli(repo : &mut Repository) {
    let matches = get_cli().get_matches();

    match matches.get_one::<bool>("author") {
        Some(true) => {println!("written by jacob (https://github.com/Oxyn4)")}
        Some(false) | None => {}
    }

    match matches.get_one::<bool>("version") {
        Some(true) => {println!("configuration manager version : 0.0.1")}
        Some(false) | None => {}
    }
    
    if matches.subcommand().is_none() {return}

    match matches.subcommand().unwrap() {
        ("add", sub_matches) => {
            let program = sub_matches.get_one::<String>("PROGRAM").expect("required"); 
            let config = sub_matches.get_one::<String>("CONFIG");
            let files = sub_matches.get_one::<String>("FILE");

            if config.is_none() && files.is_none() {
                add_command(repo, 
                    program.to_string(), 
                    None, 
                    None);
            } else if config.is_some() && files.is_none() {
                add_command(repo, 
                    program.to_string(), 
                    Some(config.unwrap().
                        to_string()), None);
            } else if config.is_some() && files.is_some() {
                add_command(
                    repo, 
                    program.to_string(), 
                    Some(config.unwrap().to_string()), 
                    Some(files.unwrap().to_string()));
            } else {
                panic!("");
            }


        } 
        ("rm", sub_matches) => {
            let program = sub_matches.get_one::<String>("PROGRAM").expect("required"); 
            let config = sub_matches.get_one::<String>("CONFIG");
            let files = sub_matches.get_one::<String>("FILE");

            if config.is_none() && files.is_none() {
                rm_command(repo, 
                    program.to_string(), 
                    None, 
                    None);
            } else if config.is_some() && files.is_none() {
                rm_command(repo, 
                    program.to_string(), 
                    Some(config.unwrap().
                        to_string()), None);
            } else if config.is_some() && files.is_some() {
                rm_command(
                    repo, 
                    program.to_string(), 
                    Some(config.unwrap().to_string()), 
                    Some(files.unwrap().to_string()));
            } else {
                panic!("");
            }
        }       
        ("ls", _) => {

            ls(repo);
        }
        (ext, _) => {
            println!("{} is not a valid use of this CLI", ext);
        }
    }
}
