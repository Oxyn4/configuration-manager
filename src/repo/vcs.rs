
pub fn get_hash_of_file(path : String) -> String {
    let file_data = std::fs::read(path).expect("could not read file data");
    let mut hasher = openssl::sha::Sha256::new();
    hasher.update(&file_data);
    let finish = hasher.finish();
    hex::encode(finish)
}

pub struct FileUpdate {
    
}

pub struct RepositoryUpdate {
    index_of_updated_program : usize,
    index_of_updated_config : Option<usize>,
    index_of_updated_file : Option<usize>,
    file_changes : Vec<FileUpdate>,
}

pub fn check_repository_for_upates(_repo : &crate::Repository) -> std::vec::Vec<RepositoryUpdate> {
    Vec::new()    
}
