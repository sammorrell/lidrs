use std::{fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub enum Error {
    EmptyFile,
    TiltNotDefined,
    TiltFileNotFound(String),
    TileFileIOError(Rc<std::io::Error>),
    TiltFiltTooLong(usize),
    InvalidKeyword(usize),
    ParseFloatError(usize, Option<usize>, std::num::ParseFloatError),
    ParseIntError(usize, Option<usize>, std::num::ParseIntError),
    InvalidUnit(usize),
    ArrayIncorrectLength(usize, usize, usize),
    VerticalAnglesInvalid(usize),
    HorizontalAnglesInvalid(usize),
    UnexpectedEndOfFile(usize),
    UnexpectedIitem(usize, usize, usize),
    FromPrimitiveError(usize, Rc<dyn std::error::Error>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                Error::TileFileIOError(ref err) => {
                    format!("Tilt file IO Error: {}", err)
                }
                Error::EmptyFile => {
                    format!("The file contains no lines. ")
                }
                Error::TiltNotDefined => {
                    format!("Unable to find TILT definition in file. ")
                }
                Error::TiltFileNotFound(ref path) => {
                    format!("Unable to load TILT file: {}", path)
                }
                Error::TiltFiltTooLong(ref len) => {
                    format!("Tilt should be 4 lines. Reached {} lines. ", len)
                }
                Error::InvalidKeyword(ref iline) => {
                    format!("Line {}: Invalid keyword. ", iline)
                }
                Error::ParseFloatError(ref iline, ref iitem, ref err) => match iitem {
                    Some(iitem) => format!(
                        "Error parsing floating point number at item {} on line {}: {}",
                        iitem, iline, err
                    ),
                    None => format!(
                        "Error parsing floating point number on line {}: {}",
                        iline, err
                    ),
                },
                Error::ParseIntError(ref iline, ref iitem, ref err) => match iitem {
                    Some(iitem) => format!(
                        "Error parsing integer number at item {} on line {}: {}",
                        iitem, iline, err
                    ),
                    None => format!("Error parsing integer number on line {}: {}", iline, err),
                },
                Error::InvalidUnit(ref iline) => {
                    format!("Line {}: Invalid unit used. ", iline)
                }
                Error::ArrayIncorrectLength(ref iline, ref expected, ref found) => {
                    format!(
                        "Line {}: Array too short. Expected {}, but found {}. ",
                        iline, expected, found
                    )
                }
                Error::VerticalAnglesInvalid(ref iline) => {
                    format!("Line {}: Vertical angles are invalid and don't conform to a hemisphere or the whole domain. ", iline )
                }
                Error::HorizontalAnglesInvalid(ref iline) => {
                    format!("Line {}: Horizontal angles are invalid and do not conform to an allowed lateral symmetry. ", iline )
                }
                Error::UnexpectedEndOfFile(ref iline) => {
                    format!("Line {}: Unexpected end of file. ", iline)
                }
                Error::UnexpectedIitem(ref iline, ref expected, ref actual) => {
                    format!(
                        "Line {}: Expected {} items, found {}. ",
                        iline, expected, actual
                    )
                }
                Error::FromPrimitiveError(ref iline, ref err) => {
                    format!("Error converting from primitive on line {}: {}", iline, err)
                }
            }
        })
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::TileFileIOError(Rc::new(err))
    }
}
