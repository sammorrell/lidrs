//! Error module.
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    IESError(crate::io::ies::Error),
    LDTError(crate::io::eulumdat::Error),
    InvalidFileType(String),
    BuildError(Box<Error>),
    OperationError(Box<crate::ops::err::Error>)
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<crate::io::ies::Error> for Error {
    fn from(err: crate::io::ies::Error) -> Self {
        Error::IESError(err)
    }
}

impl From<crate::io::eulumdat::Error> for Error {
    fn from(err: crate::io::eulumdat::Error) -> Self {
        Error::LDTError(err)
    }
}

impl From<crate::ops::err::Error> for Error {
    fn from(err: crate::ops::err::Error) -> Self {
        Error::OperationError(Box::new(err))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                Error::IOError(ref e) => format!("IO Error: {}", e),
                Error::IESError(ref e) => format!("IES Parse Error: {}", e),
                Error::LDTError(ref e) => format!("EULUMDAT (LDT) Parse Error: {}", e),
                Error::BuildError(ref err) => format!("Photometric Web Build Error: {}", err),
                Error::InvalidFileType(ref ext) => format!("Invalid file type: {}", ext),
                Error::OperationError(ref e) => format!("Operation Error: {}", e),
            }
        })
    }
}
