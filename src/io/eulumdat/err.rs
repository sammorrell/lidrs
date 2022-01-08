use std::{
    fmt::Display,
    rc::Rc,
};

#[derive(Debug, Clone)]
pub enum Error {
    TooManyLines(usize),
    ParseFloatError(usize, std::num::ParseFloatError),
    ParseIntError(usize, std::num::ParseIntError),
    InvalidUnit(usize),
    ArrayTooShort(usize, usize, usize),
    FromPrimitiveError(usize, Rc<dyn std::error::Error>)
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                Error::TooManyLines(ref iline) => {
                    format!("Line {}: The file contains too many lines", iline)
                }
                Error::InvalidUnit(ref iline) => {
                    format!("Line {}: Invalid unit used. ", iline)
                }
                Error::ArrayTooShort(ref iline, ref expected, ref found) => {
                    format!(
                        "Line {}: Array too short. Expected {}, but found {}. ",
                        iline, expected, found
                    )
                },
                Error::ParseFloatError(ref iline, ref err) => {
                    format!("Error parsing floating point number on line {}: {}", iline, err)
                },
                Error::ParseIntError(ref iline, ref err) => {
                    format!("Error parsing integer number on line {}: {}", iline, err)
                },
                Error::FromPrimitiveError(ref iline, ref err) => {
                    format!("Error converting from primitive on line {}: {}", iline, err)
                }
            }
        })
    }
}