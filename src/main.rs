
use std::io::stdout;

use crossterm::ExecutableCommand;
use log::info;

mod startup;

mod repo;
use repo::Repository;

fn add_command(repo : &mut Repository, program_name : String, config_name : Option<String>, file : Option<String>) {
    
    if file.is_some() && !std::path::Path::new(&file.clone().unwrap()).exists() {
        println!("provided file does not exist!");
        std::process::exit(0);
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

    if file.is_some() && file.is_some() {
        println!("adding file: {}", file.clone().unwrap());
        repo.new_file(program_name, config_name.unwrap(), file.unwrap());
    }
   
}

fn rm_command(repo : &mut Repository, program_name : String, config_name : Option<String>, file : Option<String>) {
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

fn show_usage() {
    println!("\na tool for managing dotfiles and configuration files\n");

    stdout().execute(crossterm::style::SetAttribute(crossterm::style::Attribute::Underlined)).unwrap();
    println!("COMMANDS");
    stdout().execute(crossterm::style::SetAttribute(crossterm::style::Attribute::NoUnderline)).unwrap();
    println!();

    println!("ls - list repository contents, show programs, configs and files");
    println!("help [command] - show detailed imformation for a specific command");
    println!("add [program name] [config name] [file path] - add programs, configs and files to repository");
    println!("rm [program name] [config name] [file path] - remove programs, configs and files from repository");
    println!();


    std::process::exit(1);
}

fn main() {
    let mode = startup::determine_mode();

    simplelog::WriteLogger::init(
        simplelog::LevelFilter::max(),
        simplelog::Config::default(),
        startup::logging::setup_log_file(mode.clone())
    ).expect("failed to intialise logger!");

    match mode {
        startup::Mode::Installed => {
            info!("running in installed mode");
        }, 
        startup::Mode::Portable => {
            info!("running in portable mode");
        }
    }
    
    info!("users home path: {}", startup::users::get_current_user().unwrap().home_dir);

    let mut repo = Repository::new(startup::users::get_current_user().unwrap().home_dir + "/.conman/").unwrap();
    
    let arguements : Vec<String> = std::env::args().collect();

    if arguements.len() == 1 {
        show_usage();
        std::process::exit(1);
    }

    log::info!("program launched as: {:?}", arguements);
    
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


            add_command(&mut repo, program_name, config_name, file_path);
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

            rm_command(&mut repo, program_name, config_name, file_path);
        },
        _ => {show_usage()}
    }

    repo.write_manifests();
}
