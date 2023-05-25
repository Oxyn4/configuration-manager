use crate::repo::Repository;




pub fn switch_command(repo : &mut Repository, program : String, config : String) {
    println!("switch called with: {}/{}", program, config);

    let pi = repo.get_program_index(program);

    let ci = repo.get_config_index(program, config);

    repo.managed_programs[pi].
}
