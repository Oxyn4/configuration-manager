
use std::fmt;

pub enum ErrorKind {
    UserEnvNotSet,
    HomeEnvNotSet,
    UserNotFound,
}

// Implement std::fmt::Display for AppError
impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "An Error Occurred, Please Try Again!") // user-facing output
        match self {
            ErrorKind::UserEnvNotSet => {
                write!(f, "the $USER enviroment variable is not set") 
            },
            ErrorKind::HomeEnvNotSet => {
                write!(f, "the $HOME enviroment variable is not set") 
            }
            ErrorKind::UserNotFound => {
                write!(f, "the user enviroment variable does not name a valid user") 
            }
        }
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "repo::error file: {}, line: {} reason: {}", file!(), line!(), self) // programmer-facing output
    }
}


