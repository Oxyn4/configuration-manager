
mod error;

#[derive(Clone)]
pub struct UsersInfo {
    pub name : String,
    pub uid : String,
    pub gid : String,
    pub gecos : String,
    pub home_dir : String,
    pub login_shell : String
}

pub fn get_users() -> Vec<UsersInfo> {
    let mut result : Vec<UsersInfo> = Vec::new();

    let passwd_data = std::fs::read("/etc/passwd").unwrap();

    let passwd : String = std::string::String::from_utf8_lossy(&passwd_data).to_string();

    let user_strings = passwd.split("\n");

    for user_data in user_strings {
        let user_pieces : Vec<&str> = user_data.split(":").collect();

        if user_pieces.len() == 7 {
            result.push(UsersInfo{
                name : user_pieces[0].to_string(),
                uid : user_pieces[2].to_string(),
                gid : user_pieces[3].to_string(),
                gecos : user_pieces[4].to_string(),
                home_dir : user_pieces[5].to_string(),
                login_shell : user_pieces[6].to_string()
            }) 
        }
    } 
    

    return result;
}

pub fn get_current_user() -> Result<UsersInfo, error::ErrorKind> {
    let all = get_users();

    let user_env = std::env::var("USER");

    if user_env.is_err() {
        unsafe {
            let user = libc::getlogin();
    
            let mut username = std::string::String::new();

            if user.is_null() {
                println!("could not determine user!");
                std::process::exit(0);
            } else {
                for c in 0 .. libc::_SC_LOGIN_NAME_MAX {
                    username.push(*user.offset(c as isize) as u8 as char);
                }
            }

            for user in &all {
                if user.name == username {
                    return Ok(user.to_owned());
                }
            }
        }
    }

    for user in all {
        if user.name == user_env.clone().unwrap() {
            return Ok(user);
        }
    }

    return Err(error::ErrorKind::UserNotFound);
}

pub fn get_home_dir() -> Option<String> {
    return Some(get_current_user().unwrap().home_dir);
}

#[derive(Clone)]
pub enum Mode {
    Installed,
    Portable
}

pub fn determine_mode() -> Mode {
    let exe_path_result = std::env::current_exe();
    
    if exe_path_result.is_err() {
        log::error!("failed to get the path to current exe resorting to portable mode!");

        println!("resorting to portable mode!");

        return Mode::Portable;
    }
    
    let exe_name_string = exe_path_result.as_ref().unwrap()
        .file_name().unwrap()
        .to_str().unwrap().to_owned();

    let exe_path_string = std::string::String::from(exe_path_result.as_ref().unwrap().to_str().unwrap());
    
    if exe_path_string == ("/usr/bin/".to_owned() + exe_name_string.as_ref()) {
        log::info!("installed in /usr/bin");

        return Mode::Installed;
    }

    let users_info = get_users();

    for user in users_info {
        let current_users_local_bin_path = user.home_dir.clone() + "/.local/bin/" + &exe_name_string.clone();

        if current_users_local_bin_path == exe_path_string {
            log::info!("detected running in users: {} ~/.local/bin/ path", user.name);
            return Mode::Installed;
        }

        let current_users_cargo_bin_path = user.home_dir.clone() + ".cargo/bin/" + &exe_path_string.clone();

        if current_users_local_bin_path == exe_path_string {
            log::info!("detected running in users: {} ~/.cargo/bin path", user.name);
            return Mode::Installed;
        }
    }
 
    Mode::Portable
}

pub fn setup_log_file(conman_mode : Mode) -> std::fs::File {
    match conman_mode {
        Mode::Installed => {
            let log_dir = std::path::Path::new("");

            todo!("setup log file for installed mode");  
        },
        Mode::Portable => {
            let mut exe_dir = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();

            exe_dir.push("logs/");

            std::fs::create_dir(exe_dir.clone());

            exe_dir.push("conman.log");

            return std::fs::File::create(exe_dir).unwrap();

        }
    }
}

