
use crate::stdout;

use crossterm::ExecutableCommand;

pub fn show_usage() {
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


