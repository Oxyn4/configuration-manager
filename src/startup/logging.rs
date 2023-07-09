
use crate::startup::*;

pub fn new_log_file_path(conman_mode : Mode) -> std::string::String {
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

            // std::fs::File::create(log_dir).unwrap()

            return log_dir.as_os_str().to_str().unwrap().to_string();
        },
        Mode::Portable => {
            let mut exe_dir = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();

            exe_dir.push("logs/");

            if !exe_dir.exists() {
                std::fs::create_dir(exe_dir.clone()).expect("failed to create logs directory");
            }

            exe_dir.push("conman_".to_owned() + &date + "_" + &time + ".log");

            // std::fs::File::create(exe_dir).unwrap()

            return exe_dir.as_os_str().to_str().unwrap().to_string();
        }
    }
}

fn return_older_log_file(file1 : String, file2 : String) {
    let log_file1_components = file1.split("_");
    let log_file2_components = file2.split("_");
    print!("{:?}", log_file1_components);
    print!("{:?}", log_file2_components);
}

pub fn logging_teardown(mode : &Mode) {
    let log_path = std::path::Path::new(&new_log_file_path(mode.clone()))
        .parent().unwrap()
        .as_os_str().to_str().unwrap()
        .to_string();

    let log_files = std::fs::read_dir(std::path::Path::new(&log_path)).unwrap().collect::<Result<Vec<_>, std::io::Error>>().unwrap();

    // 21 cause there is a log file from this program instance
    if log_files.len() > 21 {
        print!("log files need to be cleared! {}: {} log files\n", log_path, log_files.len());
    }
    
    let sorted_log_files : Vec<std::string::String> = Vec::new();

    
}

pub fn logging_init(mode : &Mode) {
    simplelog::WriteLogger::init(
        simplelog::LevelFilter::max(),
        simplelog::Config::default(),
        std::fs::File::create(new_log_file_path(mode.clone())).unwrap()
    ).expect("failed to intialise logger!");
}
