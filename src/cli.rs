
mod add;
use add::add_command;

mod rm;
use rm::rm_command;

mod ls;
use ls::ls;

use crate::repo::*;



use clap::*;

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
                .alias("a")
                .alias("ad")
                .about("add a program, config or files to the repo")
                .arg(arg!(<PROGRAM>  "the name of a program, will create a new one if does not already exist"))
                .arg(arg!([CONFIG] "the name of a config, will create a new one if does not already exist"))
                .arg(arg!([FILE] "a relitive path to a file that will be added to the specified program and config").num_args(1..))
        )
        .subcommand(
            clap::Command::new("rm")
                .alias("r")
                .about("remove a program, config or files to the repo")
                .arg(arg!(<PROGRAM>  "the name of a program to remove")) 
                .arg(arg!([CONFIG] "the name of a config to remove from the specified program"))
                .arg(arg!([FILE] "a relitive path to a file that will be removed from the specified program and config"))
        )
        .subcommand(
            clap::Command::new("ls")
                .alias("l")
                .about("list the contents of a repository")                       
        )

    

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
            let files = sub_matches.get_many::<String>("FILE");
            
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
                    Some(files.unwrap().map(|sr| sr.to_string()).collect()));
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
            println!("{ext} is not a valid use of this CLI");
        }
    }
}
