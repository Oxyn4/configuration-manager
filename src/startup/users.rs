
use crate::startup::error;

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

    let user_strings = passwd.split('\n');

    for user_data in user_strings {
        let user_pieces : Vec<&str> = user_data.split(':').collect();

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
    

    result
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

    Err(error::ErrorKind::UserNotFound)
}


pub fn get_home_dir() -> Option<String> {
    Some(get_current_user().unwrap().home_dir)
}


