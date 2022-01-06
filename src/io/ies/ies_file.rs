use crate::err::Error;
use property::Property;
use std::{
    default::{Default},
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[derive(Debug, PartialEq, Eq)]
pub enum LuminousOpeningUnits {
    Feet = 1,
    Meters = 2,
}

impl Default for LuminousOpeningUnits {
    fn default() -> Self {
        LuminousOpeningUnits::Meters
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Property)]
pub struct IESFile {
    test_number: String,
    manufacturer: String,
    rated_lumens: Option<f32>,
    candela_multiplying_factor: f32,
    n_vertical_angles: usize,
    n_horizontal_angles: usize,

    // Luminous opening parameters.
    luminous_opening_units: LuminousOpeningUnits,
    luminous_opening_width: f32,
    luminous_opening_length: f32,
    luminous_opening_height: f32,

    // Angles
    vertical_angles: Vec<f32>,
    horizontal_angles: Vec<f32>,

    // Brightness vaulues, measured in candella.
    candela_values: Vec<f32>,
}

impl IESFile {
    /// Returns a new instance of an IES file with default values.
    pub fn new() -> IESFile {
        IESFile {
            ..Default::default()
        }
    }

    /// A wrapper around the parsing code, that opens a file and reads it.
    pub fn parse_file(filepath: &Path) -> Result<IESFile, Error> {
        let infile = File::open(filepath)?;
        let mut ies_string_buf = String::new();
        BufReader::new(infile).read_to_string(&mut ies_string_buf)?;
        Self::parse(&ies_string_buf)
    }

    /// Attempts to parse an input file.
    pub fn parse(ies_string: &String) -> Result<IESFile, Error> {
        let mut ies_file = IESFile::new();
        let errors: Vec<crate::io::ies::Error> = ies_string
            .lines()
            .into_iter()
            .enumerate()
            .map(|(iline, line)| ies_file.process_line(iline, line.to_owned()))
            .filter_map(|res| {
                if res.is_err() {
                    Some(res.unwrap_err())
                } else {
                    None
                }
            })
            .collect();

        match errors.first() {
            Some(e) => Err(Error::IESError(e.clone())),
            None => Ok(ies_file),
        }
    }

    pub fn process_line(
        &mut self,
        iline: usize,
        line: String,
    ) -> Result<(), crate::io::ies::Error> {
        match iline {
            0 => Ok(()), // Nothing useful on this line.
            // The test report number of this data.
            1 => {
                self.test_number = line.replace("[TEST]", "").trim().to_owned();
                Ok(())
            }
            // The Manufacturer of the lumminaire.
            2 => {
                self.manufacturer = line.replace("[MANUFAC]", "").trim().to_owned();
                Ok(())
            }
            3 => Ok(()),
            4 => Ok(()),
            // The initial rated lumens for the lamp used in the test or -1 if absolute photometry is used and the intensity values do not depend on different lamp ratings.
            5 => match line.parse::<f32>() {
                Ok(val) => {
                    if val == -1.0 {
                        self.rated_lumens = None;
                    } else {
                        self.rated_lumens = Some(val);
                    }
                    Ok(())
                }
                Err(e) => Err(crate::io::ies::Error::ParseFloatError(iline, e)),
            },
            //A multiplying factor for all the candela values in the file. This makes it possible to easily scale all the candela values in the file when the measuring device operates in unusual units—for example, when you obtain the photometric values from a catalog using a ruler on a goniometric diagram. Normally the multiplying factor is 1.
            6 => match line.parse::<f32>() {
                Ok(val) => {
                    self.candela_multiplying_factor = val;
                    Ok(())
                }
                Err(e) => Err(crate::io::ies::Error::ParseFloatError(iline, e)),
            },
            // The number of vertical angles in the photometric web. 
            7 => match line.parse::<u64>() {
                Ok(val) => {
                    self.n_vertical_angles = val as usize;
                    Ok(())
                },
                Err(e) => Err(crate::io::ies::Error::ParseIntError(iline, e)),
            },
            // The number of horizontal angles in the photometric web. 
            8 => match line.parse::<u64>() {
                Ok(val) => {
                    self.n_horizontal_angles = val as usize;
                    Ok(())
                },
                Err(e) => Err(crate::io::ies::Error::ParseIntError(iline, e)),
            },
            9 => Ok(()),
            // The type of unit used to measure the dimensions of the luminous opening. Use 1 for feet or 2 for meters. 
            10 => match line.parse::<u8>() {
                Ok(val) => match val {
                    1 => { self.luminous_opening_units = LuminousOpeningUnits::Feet; Ok(()) },
                    2 => { self.luminous_opening_units = LuminousOpeningUnits::Meters; Ok(()) },
                    _ => Err(crate::io::ies::Error::InvalidUnit(iline))
                },
                Err(e) => Err(crate::io::ies::Error::ParseIntError(iline, e)),
            },
            // The width, length, and height of the luminous opening. 
            11 => {
                let (vals, errs): (Vec<_>, Vec<_>) = line.split(" ").map(|str| str.parse::<f32>()).partition(Result::is_ok);
                let numbers: Vec<_> = vals.into_iter().map(Result::unwrap).collect();
                let errors: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();
                match errors.first() {
                    None => { 
                        if numbers.len() == 3 {
                            self.luminous_opening_width = numbers[0];
                            self.luminous_opening_length = numbers[1];
                            self.luminous_opening_height = numbers[2];
                            Ok(())
                        } else {
                            Err(crate::io::ies::Error::ArrayTooShort(iline, 3, numbers.len()))
                        }
                    }
                    Some(e) => Err(crate::io::ies::Error::ParseFloatError(iline, e.clone())),
                }
            }
            12 => Ok(()),
            // The set of vertical angles, listed in increasing order. If the distribution lies completely in the bottom hemisphere, the first and last angles must be 0° and 90°, respectively. If the distribution lies completely in the top hemisphere, the first and last angles must be 90° and 180°, respectively. Otherwise, they must be 0° and 180°, respectively. 
            13 => {
                let (vals, errs): (Vec<_>, Vec<_>) = line.split(" ").map(|str| str.parse::<f32>()).partition(Result::is_ok);
                let numbers: Vec<_> = vals.into_iter().map(Result::unwrap).collect();
                let errors: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();
                match errors.first() {
                    None => { 
                        if numbers.len() == self.n_vertical_angles{
                            self.vertical_angles = numbers;
                            Ok(())
                        } else {
                            Err(crate::io::ies::Error::ArrayTooShort(iline, self.n_vertical_angles, numbers.len()))
                        }
                    }
                    Some(e) => Err(crate::io::ies::Error::ParseFloatError(iline, e.clone())),
                }
            },
            // The set of horizontal angles, listed in increasing order. The first angle must be 0°. The last angle determines the degree of lateral symmetry displayed by the intensity distribution. If it is 0°, the distribution is axially symmetric. If it is 90°, the distribution is symmetric in each quadrant. If it is 180°, the distribution is symmetric about a vertical plane. If it is greater than 180° and less than or equal to 360°, the distribution exhibits no lateral symmetries. All other values are invalid. 
            14 => {
                let (vals, errs): (Vec<_>, Vec<_>) = line.split(" ").map(|str| str.parse::<f32>()).partition(Result::is_ok);
                let numbers: Vec<_> = vals.into_iter().map(Result::unwrap).collect();
                let errors: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();
                match errors.first() {
                    None => { 
                        if numbers.len() == self.n_horizontal_angles{
                            self.horizontal_angles = numbers;
                            Ok(())
                        } else {
                            Err(crate::io::ies::Error::ArrayTooShort(iline, self.n_horizontal_angles, numbers.len()))
                        }
                    }
                    Some(e) => Err(crate::io::ies::Error::ParseFloatError(iline, e.clone())),
                }
            },
            // The set of candela values. First all the candela values corresponding to the first horizontal angle are listed, starting with the value corresponding to the smallest vertical angle and moving up the associated vertical plane. Then the candela values corresponding to the vertical plane through the second horizontal angle are listed, and so on until the last horizontal angle. Each vertical slice of values must start on a new line. Long lines may be broken between values as needed by following the instructions given earlier. 
            15 => {
                let (vals, errs): (Vec<_>, Vec<_>) = line.split(" ").map(|str| str.parse::<f32>()).partition(Result::is_ok);
                let numbers: Vec<_> = vals.into_iter().map(Result::unwrap).collect();
                let errors: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();
                match errors.first() {
                    None => { 
                        if numbers.len() == self.n_horizontal_angles * self.n_vertical_angles {
                            self.candela_values = numbers;
                            Ok(())
                        } else {
                            Err(crate::io::ies::Error::ArrayTooShort(iline, self.n_horizontal_angles * self.n_vertical_angles, numbers.len()))
                        }
                    }
                    Some(e) => Err(crate::io::ies::Error::ParseFloatError(iline, e.clone())),
                }
            },
            _ => Err(crate::io::ies::Error::TooManyLines(iline)),
        }
    }
}
