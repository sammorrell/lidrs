use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    IESError(crate::io::ies::Error),
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                Error::IOError(ref e) => format!("IO Error: {}", e),
                Error::IESError(ref e) => format!("IES Parse Error: {}", e),
            }
        })
    }
}
