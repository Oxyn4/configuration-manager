
#![allow(unused)]

pub mod error;

pub mod logging;

pub mod users;

use users::*;

#[derive(Clone)]
pub enum Mode {
    Installed,
    Portable
}

pub fn determine_mode() -> Mode {
    let exe_path_result = std::env::current_exe();
    
    if exe_path_result.is_err() {
        println!("failed to get the path to current exe resorting to portable mode!");

        println!("resorting to portable mode!");

        return Mode::Portable;
    }
    
    let exe_name_string = exe_path_result.as_ref().unwrap()
        .file_name().unwrap()
        .to_str().unwrap().to_owned();

    let exe_path_string = std::string::String::from(exe_path_result.as_ref().unwrap().to_str().unwrap());
    
    if exe_path_string == ("/usr/bin/".to_owned() + exe_name_string.as_ref()) {
        println!("installed in /usr/bin");

        return Mode::Installed;
    }

    let users_info = get_users();

    for user in users_info {
        let user_home_path = std::path::Path::new(&user.home_dir.clone()).to_owned();
        let users_local_bin_path = user_home_path.join(".local/bin/".to_owned() + &exe_name_string);

        if users_local_bin_path.as_os_str().to_str().unwrap() == exe_path_string {
            // println!("detected running in users: {} ~/.local/bin/ path", user.name);
            return Mode::Installed;
        }

        let local_cargo_bin_path = user_home_path.join(".cargo/bin/".to_owned() + &exe_name_string);

        if local_cargo_bin_path.as_os_str().to_str().unwrap() == exe_path_string {
            println!("detected running in users: {} ~/.cargo/bin path", user.name);
            return Mode::Installed;
        }
    }

    println!("running in portable mode!");
    Mode::Portable
}

