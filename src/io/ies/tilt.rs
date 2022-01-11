use regex::Regex;

use super::Error;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use super::DELIMITERS_PATTERN;

/// A struct for representing tilt angles in lumminaires.
#[derive(Debug, Default)]
pub struct Tilt {
    lamp_to_lumminaire_geometry: usize,
    no_tilt_angles: usize,
    angles: Vec<f32>,
    multiplying_factors: Vec<f32>,
}

impl Tilt {
    pub fn new() -> Tilt {
        Self {
            ..Default::default()
        }
    }

    pub fn from_file(filepath: &Path) -> Result<Option<Tilt>, Error> {
        let infile = File::open(filepath)?;
        let mut tilt_string_buf = String::new();
        BufReader::new(infile).read_to_string(&mut tilt_string_buf)?;
        Tilt::parse(&tilt_string_buf)
    }

    pub fn parse(tilt_str: &str) -> Result<Option<Tilt>, Error> {
        let split_regex = Regex::new(DELIMITERS_PATTERN).unwrap();
        let mut tilt = Tilt::new();
        let errors: Vec<Result<(), Error>> = tilt_str
            .lines()
            .into_iter()
            .enumerate()
            .map(|(iline, line)| {
                match iline {
                    // Get the lamp to lumminaire geometry.
                    0 => match line.parse::<u8>() {
                        Ok(val) => {
                            tilt.lamp_to_lumminaire_geometry = val as usize;
                            Ok(())
                        }
                        Err(e) => Err(crate::io::ies::Error::ParseIntError(iline, None, e)),
                    },
                    // Get the number of tilt angles.
                    1 => match line.parse::<u8>() {
                        Ok(val) => {
                            tilt.no_tilt_angles = val as usize;
                            Ok(())
                        }
                        Err(e) => Err(crate::io::ies::Error::ParseIntError(iline, None, e)),
                    },
                    // Get the tilt angles.
                    2 => {
                        let (vals, errs): (Vec<_>, Vec<_>) = split_regex
                            .split(line)
                            .map(|str| str.parse::<f32>())
                            .partition(Result::is_ok);
                        let numbers: Vec<_> = vals.into_iter().map(Result::unwrap).collect();
                        let errors: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();
                        match errors.first() {
                            None => {
                                if numbers.len() == tilt.no_tilt_angles {
                                    tilt.angles = numbers;
                                    Ok(())
                                } else {
                                    Err(crate::io::ies::Error::ArrayIncorrectLength(
                                        iline,
                                        tilt.no_tilt_angles,
                                        numbers.len(),
                                    ))
                                }
                            }
                            Some(e) => Err(crate::io::ies::Error::ParseFloatError(
                                iline,
                                None,
                                e.clone(),
                            )),
                        }
                    }
                    // Get the multiplying factors.
                    3 => {
                        let (vals, errs): (Vec<_>, Vec<_>) = split_regex
                            .split(line)
                            .map(|str| str.parse::<f32>())
                            .partition(Result::is_ok);
                        let numbers: Vec<_> = vals.into_iter().map(Result::unwrap).collect();
                        let errors: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();
                        match errors.first() {
                            None => {
                                if numbers.len() == tilt.no_tilt_angles {
                                    tilt.multiplying_factors = numbers;
                                    Ok(())
                                } else {
                                    Err(crate::io::ies::Error::ArrayIncorrectLength(
                                        iline,
                                        tilt.no_tilt_angles,
                                        numbers.len(),
                                    ))
                                }
                            }
                            Some(e) => Err(crate::io::ies::Error::ParseFloatError(
                                iline,
                                None,
                                e.clone(),
                            )),
                        }
                    }
                    _ => Err(Error::TiltFiltTooLong(iline)),
                }
            })
            .filter(Result::is_err)
            .collect();

        match errors.first() {
            None => Ok(Some(tilt)),
            Some(err) => Err(err.as_ref().unwrap_err().clone()),
        }
    }
}

impl ToString for Tilt {
    fn to_string(&self) -> String {
        format!(
            "TILT=INCLUDE\n{}\n{}\n{}\n{}\n",
            self.lamp_to_lumminaire_geometry,
            self.no_tilt_angles,
            self.angles
                .iter()
                .fold("".to_string(), |accum, val| accum + &format!("{} ", val)),
            self.multiplying_factors
                .iter()
                .fold("".to_string(), |accum, val| accum + &format!("{} ", val)),
        )
    }
}
