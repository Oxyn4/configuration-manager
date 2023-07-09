
use std::fs::DirEntry;

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

pub fn logging_teardown(mode : &Mode) {
    let log_path = std::path::Path::new(&new_log_file_path(mode.clone()))
        .parent().unwrap()
        .as_os_str().to_str().unwrap()
        .to_string();

    let mut log_files = std::fs::read_dir(std::path::Path::new(&log_path)).unwrap().collect::<Result<Vec<_>, std::io::Error>>().unwrap();

    // 21 cause there is a log file from this program instance
    if log_files.len() > 21 {
        print!("log files need to be cleared! {}: {} log files\n", log_path, log_files.len());
    }
    
    log_files.sort_by(|a, b| {
        let a = a.path().to_str().unwrap().to_string();
        let b = b.path().to_str().unwrap().to_string();
        let a_components : Vec<String> = a.split("_")
            .map(|a| {a.to_string()})
            .collect();
        
        let b_components : Vec<String> = b.split("_")
            .map(|b| {b.to_string()})
            .collect();
        
        // print!("a: {:?}\n", a_components);
        // print!("b: {:?}\n", b_components);

        let a_date_components : Vec<u32> = a_components[1].split('-').map(|a| {return a.parse::<u32>().unwrap()}).collect();
        let b_date_components : Vec<u32> = b_components[1].split('-').map(|b| {return b.parse::<u32>().unwrap()}).collect();

        let LOG_FILE_OLDER = std::cmp::Ordering::Less;
        let LOG_FILE_YOUNGER = std::cmp::Ordering::Greater;
        
        // 2021 < 2023
        if a_date_components[0] < b_date_components[0] {
            return LOG_FILE_OLDER;
        } else if a_date_components[0] == b_date_components[0] {
            if a_date_components[1] < b_date_components[1] {
                return LOG_FILE_OLDER;
            } else if a_date_components[1] == b_date_components[1] {
                if a_date_components[2] < b_date_components[2] {
                    return LOG_FILE_OLDER;
                } else {return LOG_FILE_YOUNGER;}
            } else {return LOG_FILE_YOUNGER;}
        } else {return LOG_FILE_YOUNGER;} 
        
        a_components[2].replace(".log", "");
        b_components[2].replace(".log", "");

        let a_time_components : Vec<u32> = a_components[2].split(':').map(|a| {return a.parse::<u32>().unwrap()}).collect();
        let b_time_components : Vec<u32> = b_components[2].split(':').map(|b| {return b.parse::<u32>().unwrap()}).collect();

        //hours
        // 00-23:59
        if a_time_components[0] < b_time_components[0] {
            return LOG_FILE_OLDER;
        } else if a_time_components[0] == b_time_components[0] {
            if a_time_components[1] < b_time_components[1] {
                return LOG_FILE_OLDER;
            } else if  a_time_components[1] == b_time_components[1] {
                if a_time_components[2] < b_time_components[2] {
                    return LOG_FILE_OLDER;
                } else {return LOG_FILE_YOUNGER;}
            } else {return LOG_FILE_YOUNGER;}
        } else {return  LOG_FILE_YOUNGER;}
    });

    for d in 0..(log_files.len()-20) {
        std::fs::remove_file(log_files[d].path());
    }
}

pub fn logging_init(mode : &Mode) {
    simplelog::WriteLogger::init(
        simplelog::LevelFilter::max(),
        simplelog::Config::default(),
        std::fs::File::create(new_log_file_path(mode.clone())).unwrap()
    ).expect("failed to intialise logger!");
}
