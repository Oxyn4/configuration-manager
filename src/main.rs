


use log::info;

mod startup;

mod repo;
use repo::Repository;

mod cli;

fn main() -> std::process::ExitCode {

    let mode = startup::determine_mode();
    
    startup::logging::logging_init(&mode);

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

    cli::cli(&mut repo);
   
    repo.write_manifests();

    startup::logging::logging_teardown(&mode);

    std::process::ExitCode::SUCCESS
}
