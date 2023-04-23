
use std::fmt;

pub enum ErrorKind {
    ProgramNotInRepository,
    ConfigurationNotInRepository,
    FileNotInRepository,
    MissingManifest,
    ProgramNameContainsIllegalCharacter,
    ConfigNameContainsIllegalCharacter
}

// Implement std::fmt::Display for AppError
impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "An Error Occurred, Please Try Again!") // user-facing output
        match self {
            ErrorKind::ProgramNotInRepository => {
                write!(f, "the program given is not managed by the repository") 
            },
            ErrorKind::ConfigurationNotInRepository => {
                write!(f, "the configuration given is not managed by the program given") 
            }
            ErrorKind::FileNotInRepository => {
                write!(f, "the file given is not managed by the configuration given")
            }
            ErrorKind::MissingManifest => {
                write!(f, "the configuration directory is missing a manifest.json")
            }
            ErrorKind::ConfigNameContainsIllegalCharacter => {
                write!(f, "configuration name contains an illegal character")
            },
            ErrorKind::ProgramNameContainsIllegalCharacter => {
                write!(f, "the program name contains an illegal character")
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


