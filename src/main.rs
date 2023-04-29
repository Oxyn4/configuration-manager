
use std::io::stdout;

use log::info;

mod startup;

mod repo;
use repo::Repository;

mod cli;
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

    cli::handle_arguments(&mut repo);
   
    repo.write_manifests();
}
