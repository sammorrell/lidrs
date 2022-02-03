use super::err as ies_err;
use super::lum_opening::IesLuminousOpening;
use super::{phot_type::IesPhotometryType, standard::IesStandard, tilt::Tilt};
use crate::photweb::{Plane, mirror_first_quadrant, mirror_first_hemisphere};
use crate::{
    err::Error,
    photweb::{IntensityUnits, PhotometricWeb, PhotometricWebReader, PlaneOrientation},
};
use property::Property;
use regex::Regex;
use std::{
    collections::HashMap,
    default::Default,
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
    rc::Rc,
    f64::consts::{PI, FRAC_PI_2}
};

pub const DELIMITERS_PATTERN: &str = "[ ]+|,|[\r\n]";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LuminousOpeningUnits {
    Feet = 1,
    Meters = 2,
}

impl Default for LuminousOpeningUnits {
    fn default() -> Self {
        LuminousOpeningUnits::Meters
    }
}

impl From<usize> for LuminousOpeningUnits {
    fn from(val: usize) -> Self {
        match val {
            1 => Self::Feet,
            2 => Self::Meters,
            _ => Self::default(),
        }
    }
}

impl std::fmt::Display for LuminousOpeningUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Feet => "1",
                Self::Meters => "2",
            }
        )
    }
}

#[allow(dead_code)]
#[derive(Default, Clone, Debug, Property)]
pub struct IesFile {
    standard: IesStandard,
    keywords: HashMap<String, String>,
    tilt: Option<Tilt>,

    // First line of parameters
    n_lamps: usize,
    lumens_per_lamp: f64,
    candela_multiplying_factor: f64,
    n_vertical_angles: usize,
    n_horizontal_angles: usize,
    photometric_type: IesPhotometryType,
    luminous_opening_units: LuminousOpeningUnits,
    luminous_opening_width: f64,
    luminous_opening_length: f64,
    luminous_opening_height: f64,

    // Second line of parameters.
    ballast_factor: f64,
    input_watts: f64,

    // Angles
    vertical_angles: Vec<f64>,
    horizontal_angles: Vec<f64>,

    // Brightness vaulues, measured in candellas.
    candela_values: Vec<f64>,
}

impl IesFile {
    /// Returns a new instance of an IES file with default values.
    pub fn new() -> IesFile {
        IesFile {
            ..Default::default()
        }
    }

    /// A wrapper around the parsing code, that opens a file and reads it.
    pub fn parse_file(filepath: &Path) -> Result<IesFile, Error> {
        let infile = File::open(filepath)?;
        let mut ies_string_buf = String::new();
        BufReader::new(infile).read_to_string(&mut ies_string_buf)?;
        let mut ies_file = IesFile::new();
        ies_file.parse(&ies_string_buf)?;
        Ok(ies_file)
    }

    /// Attempts to parse an input file.
    pub fn parse(&mut self, ies_string: &String) -> Result<(), Error> {
        let standard = match ies_string.lines().into_iter().nth(0) {
            None => Err(Error::IESError(ies_err::Error::EmptyFile)),
            Some(val) => Ok(IesStandard::from(val)),
        };

        // If at this point we have an error, just return it. Else we can continue.
        if standard.is_err() {
            return Err(standard.unwrap_err());
        }
        self.standard = standard.unwrap();

        // Parse the keywords.
        self.parse_keywords(&ies_string)?;

        // Parse the TILT.
        self.parse_tilt(&ies_string)?;

        // Now get he remaining values.
        self.parse_properties(&ies_string)?;

        Ok(())
    }

    /// Parses the keywords section of the file.
    pub fn parse_keywords(&mut self, ies_string: &String) -> Result<(), ies_err::Error> {
        // First we find the start line, if not 1986 standard, this will be after the first line.
        let start = if self.standard == IesStandard::Iesna1986 {
            0
        } else {
            1
        };
        // Now find the ending of the keyword section. We can guarantee the line after will always start with "TILT=".
        let end = ies_string
            .lines()
            .position(|line| line.starts_with("TILT="));
        if end.is_none() {
            return Err(ies_err::Error::TiltNotDefined);
        }

        // Build the Regex for Keywork matching.
        let kw_regex = Regex::new("\\[([A-Z_]+)\\] (.*)").unwrap();

        // Get those lines and iterate through them.
        let (keywords, errors): (
            Vec<Result<(String, String), ies_err::Error>>,
            Vec<Result<(String, String), ies_err::Error>>,
        ) = ies_string
            .lines()
            .into_iter()
            .enumerate()
            .skip(start)
            .take(end.unwrap() - start)
            .map(|(iline, line)| {
                // Get the keyword.
                let cap = kw_regex.captures_iter(line);
                match cap.into_iter().nth(0) {
                    None => Err(ies_err::Error::InvalidKeyword(start + iline + 1)),
                    Some(kw) => {
                        // We have a keyword - data pair.
                        Ok((
                            kw.get(1).unwrap().as_str().to_owned(),
                            kw.get(2).unwrap().as_str().to_owned(),
                        ))
                    }
                }
            })
            .partition(Result::is_ok);

        let mut previous_kw: Option<String> = None;
        match errors.first() {
            None => {
                for vals in keywords {
                    let kw = vals.unwrap();

                    if kw.0 == "MORE" {
                        self.keywords
                            .get_mut(previous_kw.as_ref().unwrap())
                            .unwrap()
                            .push_str(&format!(" {}", kw.1));
                    } else {
                        previous_kw = Some(kw.0.clone());
                        self.keywords.insert(kw.0, kw.1);
                    }
                }
                Ok(())
            }
            Some(err) => Err(err.as_ref().unwrap_err().clone()),
        }
    }

    pub fn parse_tilt(&mut self, ies_string: &String) -> Result<(), ies_err::Error> {
        let tilt_res = match ies_string
            .lines()
            .position(|line| line.starts_with("TILT="))
        {
            None => Err(ies_err::Error::TiltNotDefined),
            Some(val) => {
                match ies_string
                    .lines()
                    .nth(val)
                    .unwrap()
                    .replace("TILT=", "")
                    .as_str()
                {
                    "NONE" => Ok(None),
                    "INCLUDE" => {
                        // Pick off just the 4 lines we're interested in and parse.
                        let tilt_lines = ies_string
                            .lines()
                            .skip(val + 1)
                            .take(4)
                            .fold("".to_string(), |accum, item| format!("{}{}\n", accum, item));
                        Tilt::parse(tilt_lines.as_str())
                    }
                    // In this case, we are being given a filename.
                    _ => {
                        let tilt_line = ies_string.lines().nth(val).unwrap();
                        Tilt::from_file(&Path::new(&tilt_line.replace("TILT=", "").to_owned()))
                    }
                }
            }
        };

        match tilt_res {
            Ok(tilt) => {
                self.tilt = tilt;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// This function reads the properties from the file into the data structure.
    pub fn parse_properties(&mut self, ies_string: &String) -> Result<(), ies_err::Error> {
        // I will likely revisit this in the future as I'm unhappy with how this is implemented.
        // It is implemented in a really awkward way. I would like to do this in a nicer way, but
        // I am in a rush and I need it to be working.
        let split_regex = Regex::new(DELIMITERS_PATTERN).unwrap();

        // Assemble and parse all of the numbers.
        let tilt_end_res = ies_string
            .lines()
            .position(|line| line.starts_with("TILT="));
        if tilt_end_res.is_none() {
            return Err(ies_err::Error::TiltNotDefined);
        };

        let tilt_skip = match ies_string
            .lines()
            .nth(tilt_end_res.unwrap())
            .unwrap()
            .replace("TILT=", "")
            .as_str()
        {
            "INCLUDE" => 5,
            _ => 1,
        };

        // Read all of the parameters as one long array, as we know the order and number.
        let start_line = tilt_end_res.unwrap() + tilt_skip;
        let lines: Vec<(usize, String)> = ies_string
            .lines()
            .skip(start_line)
            .enumerate()
            .map(|(iline, str)| {
                let tmp: Vec<(usize, String)> = split_regex
                    .split(str.trim())
                    .map(|val_str| (start_line + iline + 1, String::from(val_str)))
                    .collect();
                tmp
            })
            .flatten()
            .collect();

        let errs: Vec<ies_err::Error> = lines
            .iter()
            .enumerate()
            .map(|(iitem, (iline, item))| {
                match iitem {
                    0 => match item.parse() {
                        Ok(val) => {
                            self.n_lamps = val;
                            Ok(())
                        }
                        Err(err) => {
                            Err(ies_err::Error::ParseIntError(*iline, Some(iitem + 1), err))
                        }
                    },
                    1 => match item.parse() {
                        Ok(val) => {
                            self.lumens_per_lamp = val;
                            Ok(())
                        }
                        Err(err) => Err(ies_err::Error::ParseFloatError(
                            *iline,
                            Some(iitem + 1),
                            err,
                        )),
                    },
                    2 => match item.parse() {
                        Ok(val) => {
                            self.candela_multiplying_factor = val;
                            Ok(())
                        }
                        Err(err) => Err(ies_err::Error::ParseFloatError(
                            *iline,
                            Some(iitem + 1),
                            err,
                        )),
                    },
                    3 => match item.parse() {
                        Ok(val) => {
                            self.n_vertical_angles = val;
                            Ok(())
                        }
                        Err(err) => {
                            Err(ies_err::Error::ParseIntError(*iline, Some(iitem + 1), err))
                        }
                    },
                    4 => match item.parse() {
                        Ok(val) => {
                            self.n_horizontal_angles = val;
                            Ok(())
                        }
                        Err(err) => {
                            Err(ies_err::Error::ParseIntError(*iline, Some(iitem + 1), err))
                        }
                    },
                    5 => match item.parse::<usize>() {
                        Ok(val) => match val.try_into() {
                            Ok(phottype) => {
                                self.photometric_type = phottype;
                                Ok(())
                            }
                            Err(err) => {
                                Err(ies_err::Error::FromPrimitiveError(*iline, Rc::new(err)))
                            }
                        },
                        Err(err) => {
                            Err(ies_err::Error::ParseIntError(*iline, Some(iitem + 1), err))
                        }
                    },
                    6 => match item.parse::<usize>() {
                        Ok(val) => {
                            self.luminous_opening_units = val.into();
                            Ok(())
                        }
                        Err(err) => {
                            Err(ies_err::Error::ParseIntError(*iline, Some(iitem + 1), err))
                        }
                    },
                    7 => match item.parse() {
                        Ok(val) => {
                            self.luminous_opening_width = val;
                            Ok(())
                        }
                        Err(err) => Err(ies_err::Error::ParseFloatError(
                            *iline,
                            Some(iitem + 1),
                            err,
                        )),
                    },
                    8 => match item.parse() {
                        Ok(val) => {
                            self.luminous_opening_length = val;
                            Ok(())
                        }
                        Err(err) => Err(ies_err::Error::ParseFloatError(
                            *iline,
                            Some(iitem + 1),
                            err,
                        )),
                    },
                    9 => match item.parse() {
                        Ok(val) => {
                            self.luminous_opening_height = val;
                            Ok(())
                        }
                        Err(err) => Err(ies_err::Error::ParseFloatError(
                            *iline,
                            Some(iitem + 1),
                            err,
                        )),
                    },
                    10 => match item.parse() {
                        Ok(val) => {
                            self.ballast_factor = val;
                            Ok(())
                        }
                        Err(err) => Err(ies_err::Error::ParseFloatError(
                            *iline,
                            Some(iitem + 1),
                            err,
                        )),
                    },
                    11 => Ok(()),
                    12 => match item.parse() {
                        Ok(val) => {
                            self.input_watts = val;
                            Ok(())
                        }
                        Err(err) => Err(ies_err::Error::ParseFloatError(
                            *iline,
                            Some(iitem + 1),
                            err,
                        )),
                    },
                    // We will now read the vertical angles from the file.
                    i if i > 12 && i <= 12 + self.n_vertical_angles => match item.parse() {
                        Ok(val) => {
                            self.vertical_angles.push(val);
                            Ok(())
                        }
                        Err(err) => Err(ies_err::Error::ParseFloatError(
                            *iline,
                            Some(iitem + 1),
                            err,
                        )),
                    },
                    // Now read the horizontal values from the file.
                    i if i > 12 + self.n_vertical_angles
                        && i <= 12 + self.n_vertical_angles + self.n_horizontal_angles =>
                    {
                        match item.parse() {
                            Ok(val) => {
                                self.horizontal_angles.push(val);
                                Ok(())
                            }
                            Err(err) => Err(ies_err::Error::ParseFloatError(
                                *iline,
                                Some(iitem + 1),
                                err,
                            )),
                        }
                    }
                    // Now read the candela values.
                    i if i >= 12 + self.n_vertical_angles + self.n_horizontal_angles => {
                        match item.parse() {
                            Ok(val) => {
                                self.candela_values.push(val);
                                Ok(())
                            }
                            Err(err) => Err(ies_err::Error::ParseFloatError(
                                *iline,
                                Some(iitem + 1),
                                err,
                            )),
                        }
                    }
                    // If the properties are not consistent with the arrays, we want to put this here just to check.
                    _ => Err(ies_err::Error::UnexpectedIitem(
                        *iline,
                        12 + self.n_vertical_angles + self.n_horizontal_angles,
                        lines.iter().count(),
                    )),
                }
            })
            .filter_map(|res| {
                if res.is_err() {
                    Some(res.unwrap_err())
                } else {
                    None
                }
            })
            .collect();

        if !errs.is_empty() {
            return Err(errs.first().unwrap().clone());
        }

        Ok(())
    }

    /// Checks to see that the vertical angles are valid according to the IES standard,
    /// The valid configurations are:
    /// - Completely in the bottom hemisphere: first angles and 0 degrees and 90 degress respectively.
    /// - Completely in the top hemisphere: first angles and 90 degrees and 180 degress respectively.
    /// - Otherwise: first angles and 0 degrees and 180 degress respectively.
    pub fn vertical_angles_valid(angles: &Vec<f64>) -> bool {
        match angles.first() {
            Some(first) => match first {
                x if *x == 0.0 => match angles.last() {
                    None => false,
                    Some(last) => match last {
                        y if *y == 90.0 || *y == 180.0 => true,
                        _ => false,
                    },
                },
                x if *x == 90.0 => match angles.last() {
                    None => false,
                    Some(last) => match last {
                        y if *y == 180.0 => true,
                        _ => false,
                    },
                },
                _ => false,
            },
            None => false,
        }
    }

    /// Checks to see that the horizontal angles are valid according to the IES standard.
    /// In this case, the angles are mainly used to defined symmetries, the rules are:
    /// - First angle must always be 0.0.
    /// - If the last values is 0.0, the distribution is axially symmetric.
    /// - If the last value is 90.0 degress, the distribution is symmetric in each quadrant.
    /// - If the last value is 180.0 degress, the distribution is symmetric about a vertical plane.
    /// - If the last value is greater than 180.0 and less than or equal to 360.0, no lateral symmetries.
    /// - Hence, the valid last values are: 0.0, 90.0, 180.0 - 360.0.  
    pub fn horizontal_angles_valid(angles: &Vec<f64>) -> bool {
        match angles.first() {
            Some(first) => match first {
                x if *x == 0.0 => match angles.last() {
                    Some(last) => match last {
                        y if *y == 0.0 => true,
                        y if *y == 90.0 => true,
                        y if *y >= 180.0 && *y <= 360.0 => true,
                        _ => false,
                    },
                    None => false,
                },
                _ => false,
            },
            None => false,
        }
    }

    /// Writes the currently loaded EULUMDAT file to a specified file.
    /// The written value is determined by `LdtFile::to_string(&self)`.
    pub fn to_file(&self, outpath: &Path) -> Result<(), Error> {
        let mut file = File::create(outpath)?;
        file.write(self.to_string().as_bytes())?;
        Ok(())
    }

    /// Outputs the keywords in the file to a string.
    pub fn keywords_to_string(&self) -> String {
        self.keywords
            .iter()
            .fold("".to_string(), |accum, (key, val)| {
                accum + &format!("[{}] {}\n", key, val)
            })
    }

    /// Get the type and properties of the luminous opening.
    pub fn get_luminous_opening(&self) -> IesLuminousOpening {
        IesLuminousOpening::from_dimensions(
            self.luminous_opening_width,
            self.luminous_opening_length,
            self.luminous_opening_height,
        )
    }

    /// Gets the planes from this object.
    pub fn get_planes(&self) -> Vec<Plane> {
        match self.photometric_type {
            IesPhotometryType::TypeA => self.get_planes_type_a(),
            IesPhotometryType::TypeB => self.get_planes_type_b(),
            IesPhotometryType::TypeC => self.get_planes_type_c(),
        }
    }

    /// Get the planes from a Type A photometry IES file.
    pub fn get_planes_type_a(&self) -> Vec<Plane> {
        todo!()
    }

    /// Get the planes from a Type A photometry IES file.
    pub fn get_planes_type_b(&self) -> Vec<Plane> {
        todo!()
    }

    /// Get the planes from a Type C photometry IES file.
    pub fn get_planes_type_c(&self) -> Vec<Plane> {
        // Chunk the intensities into the planes, and give them appropriate angles.
        let mut planes = self
            .candela_values
            .chunks(self.n_vertical_angles)
            .enumerate()
            .map(|(iplane, intensities_candelas)| {
                let mut curr_plane = Plane::new();
                curr_plane.set_angle_degrees(self.horizontal_angles[iplane]);
                curr_plane.set_orientation(PlaneOrientation::Vertical);
                curr_plane.set_intensities(Vec::from(intensities_candelas));
                curr_plane.set_angles_degrees(&self.vertical_angles);
                curr_plane.set_units(IntensityUnits::Candela);
                curr_plane
            })
            .collect::<Vec<Plane>>();
        
        // Now resolve the symmetries.
        // First, check if we have the first quadrant filled (from 0 -> 90 deg).
        // If so, mirror this to fill the 0 -> 180 degree hemisphere. 
        if (planes.iter().last().unwrap().angle() - (PI / 2.0)).abs() <= f64::EPSILON {
            planes = mirror_first_quadrant(&planes);
        }

        // Now, check to see if we have the first hemisphere (0 -> 180 deg). 
        // If so mirror this to fill the final hemisphere.
        if (planes.iter().last().unwrap().angle() - (PI)).abs() <= f64::EPSILON {
            planes = mirror_first_hemisphere(&planes);
        }

        planes
    }
}

impl ToString for IesFile {
    fn to_string(&self) -> String {
        let mut output = String::new();

        // Get the standard header.
        let stan = self.standard.to_string();
        if !stan.is_empty() {
            output = format!("{}\n", &stan);
        };

        // Output keywords
        output += &self.keywords_to_string();

        // Output the tilt.
        let tilt_str = match &self.tilt {
            None => String::from("TILT=NONE\n"),
            Some(val) => val.to_string(),
        };
        output += &tilt_str;

        // Now output the parameters and arrays.
        output += &format!(
            "{} {} {} {} {} {} {} {} {} {}\n",
            self.n_lamps,
            self.lumens_per_lamp,
            self.candela_multiplying_factor,
            self.n_vertical_angles,
            self.n_horizontal_angles,
            (self.photometric_type.clone() as usize),
            self.luminous_opening_units,
            self.luminous_opening_width,
            self.luminous_opening_length,
            self.luminous_opening_height
        );
        output += &format!("{} {} {}\n", self.ballast_factor, 1, self.input_watts,);
        output += &format!(
            "{}\n",
            self.vertical_angles
                .iter()
                .fold(String::new(), |accum, val| accum + &format!("{} ", val))
        );
        output += &format!(
            "{}\n",
            self.horizontal_angles
                .iter()
                .fold(String::new(), |accum, val| accum + &format!("{} ", val))
        );
        output += &format!(
            "{}\n",
            self.candela_values
                .chunks(self.n_vertical_angles)
                .fold(String::new(), |accum, val| accum
                    + &format!(
                        "{}\n",
                        val.iter()
                            .fold(String::new(), |accum, val| accum + &format!("{} ", val))
                    ))
        );

        output
    }
}

impl From<IesFile> for PhotometricWeb {
    fn from(ies: IesFile) -> Self {
        let mut photweb = PhotometricWeb::new();
        photweb.set_planes(ies.get_planes());
        photweb
    }
}

//TODO: Implement conversion.
impl PhotometricWebReader for IesFile {
    fn read(&self, path: &Path) -> Result<PhotometricWeb, Error> {
        let ies_file = Self::parse_file(path)?;
        let photweb = ies_file.into();
        Ok(photweb)
    }
}
