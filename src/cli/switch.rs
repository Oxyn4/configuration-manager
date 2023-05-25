use crate::repo::Repository;




pub fn switch_command(repo : &mut Repository, program : String, config : String) {
    println!("switch called with: {}/{}", program, config);

    let pi = repo.get_program_index(program.clone()).expect("failed to get program index");

    let ci = repo.get_config_index(program, config).expect("failed to get config index");

    repo.managed_programs[pi].switch_active_config(ci);
}
