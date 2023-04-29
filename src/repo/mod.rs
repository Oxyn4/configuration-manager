
pub mod program;
use log::info;
use program::Program;

pub mod config_file;
use config_file::ConfigFile;

pub mod config;
use config::Config;

pub mod error;

mod vcs;

pub struct Repository {
    pub root : String,
    pub managed_programs : std::vec::Vec<Program>, 
}

impl Repository {
    pub fn new(repo_root : String) -> Option<Repository> {

        info!("looking for a repository with repo_root: {}", repo_root);
    
        let mut managed_programs_init = Vec::new();

        let repo_root_path = std::path::Path::new(&repo_root);
        if !repo_root_path.exists() {
            info!("the repository does not exist creating new one");
            let creation_result = std::fs::create_dir_all(repo_root.clone());

            if creation_result.is_ok() {
                info!("created repository");

                return Some(Repository {
                    root : repo_root,
                    managed_programs : Vec::new()
                })
            } else {
                info!("error creating repository");
                return None;
            }
        }

        let programs_root_path = repo_root_path.join("programs/");

        let programs_root_path_it_result = std::fs::read_dir(programs_root_path);

        if programs_root_path_it_result.is_err() {
            log::warn!("repository has no programs directory");
        } 

        let programs_root_path_it = programs_root_path_it_result.unwrap();

        for program in programs_root_path_it {
            info!("found program in programs directory path: {}", program.as_ref().unwrap().path().into_os_string().into_string().unwrap());
            
            managed_programs_init.push(Program::new(program.unwrap().path().into_os_string().into_string().unwrap()));
        }
                

        Some(Repository { 
            root: repo_root, 
            managed_programs: managed_programs_init 
        })
    }

    pub fn get_program_index(&self, program_name : String) -> Result<usize, error::ErrorKind> {
        let mut count : usize = 0;
        for p in &self.managed_programs {
            if p.name == program_name {
                return Ok(count);
            }
            count += 1;
        }
        Err(error::ErrorKind::ProgramNotInRepository)
    }

    pub fn get_config_index(&self, program_name : String, config_name : String) -> Result<usize, error::ErrorKind> {
        let pi = self.get_program_index(program_name).expect("failed to get index of program");
        let mut count : usize = 0;
        for c in &self.managed_programs[pi].conifigurations {
            if c.name == config_name {
                return Ok(count);
            }
            count += 1;
        }
        Err(error::ErrorKind::ConfigurationNotInRepository)
    }

    pub fn get_file_index(&self, program_name : String, config_name : String, file : String) -> Result<usize, error::ErrorKind> {
        let pi = self.get_program_index(program_name.clone())?;
        let ci = self.get_config_index(program_name, config_name)?;
        let mut count : usize = 0;
        let file_name = std::path::Path::new(&file).file_name().unwrap().to_owned().into_string().unwrap();
        for f in &self.managed_programs[pi].conifigurations[ci].managed_files {
            if f.file_name == file_name {
                return Ok(count);
            }
            count += 1;
        }
        Err(error::ErrorKind::FileNotInRepository)
    }

    pub fn write_manifests(&self) {
        for p in &self.managed_programs {
            for c in &p.conifigurations {
                c.write_manifest();
            }
        }

    }

    pub fn new_program(&mut self, program_name : String) -> u16 {
        let r = self.managed_programs.len() as u16;
        self.managed_programs.push(Program::new(self.root.clone() + "programs/" + &program_name + "/"));
        r
    }

    pub fn new_config(&mut self, program_name : &String, config_name : String) {
        let i = self.get_program_index(program_name.clone()).unwrap();

        self.managed_programs[i].conifigurations.push(Config::new(self.root.clone() + "programs/" + program_name + "/" + &config_name).unwrap());
    }

    pub fn new_file(&mut self, program_name : String, config_name : String, relitive_file_path : String) {
        let pi = self.get_program_index(program_name.clone()).unwrap();

        let ci = self.get_config_index(program_name.clone(), config_name.clone()).unwrap();

        let rel_file_path_clone = relitive_file_path.clone();

        let file_path = std::path::Path::new(&rel_file_path_clone); 

        let file_name = file_path.file_name().unwrap().to_owned().into_string().unwrap();

        let destination = self.root.clone() + "programs/" + &program_name + "/" + &config_name + "/" + &file_name;

        let cf = ConfigFile::new(relitive_file_path).unwrap();

        // make sure the new file is not already in config
        for f in &self.managed_programs[pi].conifigurations[ci].managed_files {
            if cf.destination_path == f.destination_path {
                println!("file managed for this config");
                return;
            }
        }

        std::fs::copy(file_path, destination).unwrap();

        self.managed_programs[pi].conifigurations[ci].managed_files.push(cf);
    }

    pub fn rm_program(&mut self, program_name : String) {
        let pi = self.get_program_index(program_name).unwrap();

        self.managed_programs[pi].delete();

        self.managed_programs.remove(pi);
    }

    pub fn rm_config(&mut self, program_name : String, config_name : String) {
        let pi = self.get_program_index(program_name.clone()).unwrap();

        let ci = self.get_config_index(program_name, config_name).unwrap();

        self.managed_programs[pi].conifigurations[ci].delete();

        self.managed_programs[pi].conifigurations.remove(ci);
    }

    pub fn rm_file(&mut self, program_name : String, config_name : String, file : String) {
        let pi = self.get_program_index(program_name.clone()).unwrap();

        let ci = self.get_config_index(program_name.clone(), config_name.clone()).unwrap();

        let fi = self.get_file_index(program_name, config_name, file).unwrap();

        std::fs::remove_file(self.managed_programs[pi].conifigurations[ci].get_directory_path() + &self.managed_programs[pi].conifigurations[ci].managed_files[fi].file_name).unwrap();

        self.managed_programs[pi].conifigurations[ci].managed_files.remove(fi);
    }
}
