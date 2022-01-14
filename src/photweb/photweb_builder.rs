use std::{default::Default, path::Path};

use super::{PhotometricWeb, PhotometricWebReader};
use crate::{err::Error, io};

/// The object that builds `PhotometricWeb` objects.
/// This can instantiate from values, or read from a file of one of the supported types.
#[derive(Default)]
pub struct PhotometricWebBuilder {
    input_file: Option<Box<Path>>,
}

impl PhotometricWebBuilder {
    /// Generated a builder instance from a file path.
    pub fn from_file(filepath: &Path) -> PhotometricWebBuilder {
        Self {
            input_file: Some(Box::from(filepath)),
            ..Default::default()
        }
    }

    /// Attempts to return the correct file parser for the given provided file.
    /// Returns a boxed reader if valid, else an `Error::InvalidFileType` error is resturned.
    pub fn get_file_parser(path: &Path) -> Result<Box<dyn PhotometricWebReader>, Error> {
        match path.extension() {
            None => Err(Error::InvalidFileType(String::new())),
            Some(file_ext) => match file_ext.to_str() {
                None => Err(Error::InvalidFileType(String::new())),
                Some(file_ext) => match file_ext {
                    "ldt" => Ok(Box::new(io::eulumdat::EulumdatFile::new())),
                    "eul" => Ok(Box::new(io::eulumdat::EulumdatFile::new())),
                    "ies" => Ok(Box::new(io::ies::IesFile::new())),
                    _ => Err(Error::InvalidFileType(file_ext.to_owned())),
                },
            },
        }
    }

    /// Attempts to build the photometric web from the provided information.
    pub fn build(&self) -> Result<PhotometricWeb, Error> {
        match &self.input_file {
            Some(box_path) => {
                let rdr = Self::get_file_parser(&*box_path)?;
                let phot = rdr.read(&*box_path)?;
                Ok(phot)
            }
            None => {
                let phot = PhotometricWeb::new();
                Ok(phot)
            }
        }
    }
}
