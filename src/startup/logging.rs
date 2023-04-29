
use crate::startup::*;

pub fn setup_log_file(conman_mode : Mode) -> std::fs::File {
    let current_time = chrono::Utc::now();
    let date = current_time.date_naive().to_string();
    let mut time = current_time.time().to_string();
    
    // remove the fractional seconds from the time
    let _ = time.split_off(current_time.clone().time().to_string().find('.').unwrap());

    match conman_mode {
        Mode::Installed => {
            let mut log_dir = std::path::Path::new(&(get_home_dir().unwrap() + "/.local/state/conman/logs")).to_owned();

            if !log_dir.exists() {
                std::fs::create_dir_all(log_dir.clone());
            }

            log_dir.push("conman_".to_owned() + &date + "_" + &time + ".log");

            std::fs::File::create(log_dir).unwrap()
        },
        Mode::Portable => {
            let mut exe_dir = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();

            exe_dir.push("logs/");

            if !exe_dir.exists() {
                std::fs::create_dir(exe_dir.clone()).expect("failed to create logs directory");
            }

            exe_dir.push("conman_".to_owned() + &date + "_" + &time + ".log");

            std::fs::File::create(exe_dir).unwrap()

        }
    }
}
