use super::err as ldt_err;
use crate::err::Error;
use property::Property;
use std::{
    default::Default,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[derive(Debug, Clone)]
pub enum LdtType {
    PointSourceNoSymmetry = 0,
    SymmetryAboutVerticalAxis = 1,
}

impl Default for LdtType {
    fn default() -> Self {
        LdtType::PointSourceNoSymmetry
    }
}

#[derive(Debug, Clone)]
pub enum LdtSymmetry {
    NoSymmetry = 0,
}

impl Default for LdtSymmetry {
    fn default() -> Self {
        LdtSymmetry::NoSymmetry
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Property)]
pub struct LdtFile {
    manufacturer: String,
    ltype: LdtType,
    symmetry: LdtSymmetry,
}

impl LdtFile {
    // Returns a new instance of an IES file with default values.
    pub fn new() -> LdtFile {
        LdtFile {
            ..Default::default()
        }
    }

    /// A wrapper around the parsing code, that opens a file and reads it.
    pub fn parse_file(filepath: &Path) -> Result<LdtFile, Error> {
        let infile = File::open(filepath)?;
        let mut ies_string_buf = String::new();
        BufReader::new(infile).read_to_string(&mut ies_string_buf)?;
        Self::parse(&ies_string_buf)
    }

    /// Attempts to parse an input file.
    pub fn parse(ies_string: &String) -> Result<LdtFile, Error> {
        todo!()
    }

    pub fn process_line(&mut self, iline: usize, _line: String) -> Result<(), ldt_err::Error> {
        match iline {
            _ => Err(ldt_err::Error::TooManyLines(iline)),
        }
    }
}
