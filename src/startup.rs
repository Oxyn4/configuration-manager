
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
            println!("detected running in users: {} ~/.local/bin/ path", user.name);
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

pub fn setup_log_file(conman_mode : Mode) -> std::fs::File {
    let current_time = chrono::Utc::now().to_rfc3339();

    match conman_mode {
        Mode::Installed => {
            let mut log_dir = std::path::Path::new(&(get_home_dir().unwrap() + "/.local/state/conman/logs")).to_owned();

            if !log_dir.exists() {
                std::fs::create_dir_all(log_dir.clone());
            }

            log_dir.push("conman_".to_owned() + &current_time + ".log");

            return std::fs::File::create(log_dir).unwrap();
        },
        Mode::Portable => {
            let mut exe_dir = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();

            exe_dir.push("logs/");

            if !exe_dir.exists() {
                std::fs::create_dir(exe_dir.clone()).expect("failed to create logs directory");
            }

            exe_dir.push("conman_".to_owned() + &current_time + ".log");

            return std::fs::File::create(exe_dir).unwrap();

        }
    }
}
