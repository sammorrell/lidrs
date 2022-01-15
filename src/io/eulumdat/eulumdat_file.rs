use super::err as ldt_err;
use super::{util, EulumdatSymmetry, EulumdatType};
use crate::photweb::{IntensityUnits, PhotometricWeb, Plane, PlaneOrientation};
use crate::util::geom::degrees_to_radians;
use crate::{err::Error, photweb::PhotometricWebReader};
use property::Property;
use std::f64::consts::{FRAC_2_PI, FRAC_PI_2, PI};
use std::{
    default::Default,
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
    rc::Rc,
};

/// The line on which the the lamp section starts, this should be after the TILT section.
const LAMP_SECTION_START: usize = 27;
/// The total number of different parameters being read in for each lamp set.
const N_LAMP_PARAMS: usize = 6;

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Property, PartialEq)]
pub struct EulumdatFile {
    /// The first line of the file. Contains company identification / data bank / version / format identification.
    header: String,
    /// The type indicator.
    ltype: EulumdatType,
    /// The symmetry indicator.
    symmetry: EulumdatSymmetry,

    /// Mc - Number of C-Planes between 0 - 360 degrees.
    n_cplanes: usize,
    /// Dc - distance between CPlanes.
    cplane_dist: f64,
    /// Ng - Number of luminous intensities between C-Planes.
    n_luminous_intensities_per_cplane: usize,
    /// Dg - Distance between luminous intensities per C-Plane.
    distance_between_luminous_intensities_per_cplane: f64,
    /// Measurement report number.
    measurement_report_number: String,
    /// The name of the lumminaire.
    luminaire_name: String,
    /// The lumminaire number.
    luminaire_number: String,
    /// The filename of the file (as provided by the file).
    filename: String,
    /// The date / user that created the file.
    date_user: String,

    /** The dimensions of the lumminaire (mm). **/
    /// Length (or diameter) of the lumminaire in mm.
    luminaire_length: f64,
    /// Width of the luminaire (0 for circular luminaire).
    luminaire_width: f64,
    /// Height of the luminaire.
    luminaire_height: f64,

    /** The dimensions of the luminous area (mm). **/
    /// Length (or diameter) of the luminous area in mm.
    luminous_area_length: f64,
    /// Width of the luminous area (0 for circular luminous area).
    luminous_area_width: f64,
    /// Height of the luminous area of the C0-plane.
    luminous_area_height_c0: f64,
    /// Height of the luminous area of the C90-plane.
    luminous_area_height_c90: f64,
    /// Height of the luminous area of the C180-plane.
    luminous_area_height_c180: f64,
    /// Height of the luminous area of the C270-plane.
    luminous_area_height_c270: f64,

    /// Downward flux fraction (DFF) in percent.
    downward_flux_fraction: f64,
    /// Light output ratio luminaire (LORL) in percent.
    light_output_ratio_luminaire: f64,
    /// Luminous intensity conversation factor.
    luminous_intensity_conversion_factor: f64,
    /// The tilt of the lumminaire during measurement.
    tilt: f64,

    /** Lamp Parameters **/
    /// n - Number of sets of standard lamps.
    n_lamp_sets: usize,
    /// The number of lamps in each set.
    n_lamp: Vec<i32>,
    /// The type of the lamp.
    lamp_type: Vec<String>,
    /// Total luminous flux of lamps (lumens)
    tot_luminous_flux: Vec<f64>,
    /// Colour appearance / color temperature of lamps
    color_temperature: Vec<String>,
    /// Colour rendering group.
    color_rendering_group: Vec<String>,
    /// Wattage including ballast (watts).  
    wattage: Vec<f64>,

    /// Direct ratios for room indices.
    direct_ratios: Vec<f64>,

    /// C-angles.
    c_angles: Vec<f64>,
    /// G-Angles
    g_angles: Vec<f64>,
    /// Luminous intensities.
    intensities: Vec<f64>,
}

impl EulumdatFile {
    // Returns a new instance of an IES file with default values.
    pub fn new() -> EulumdatFile {
        EulumdatFile {
            ..Default::default()
        }
    }

    /// A wrapper around the parsing code, that opens a file and reads it.
    pub fn parse_file(filepath: &Path) -> Result<EulumdatFile, Error> {
        let infile = File::open(filepath)?;
        let mut ldt_string_buf = String::new();
        BufReader::new(infile).read_to_string(&mut ldt_string_buf)?;
        let mut ldt = EulumdatFile::new();
        ldt.parse(&ldt_string_buf)?;
        Ok(ldt)
    }

    /// Attempts to parse an input file.
    pub fn parse(&mut self, ldt_string: &String) -> Result<(), Error> {
        // Get all of the lines as a Vec, trimming the whitespace where required.
        let lines: Vec<(usize, String)> = ldt_string
            .lines()
            .enumerate()
            .map(|(iline, str)| (iline + 1, String::from(str.trim())))
            .collect();

        // Now parse each of the lines of the file in turn into the struct.
        // As we go along, we will return `Ok(())` if all is good, or an error
        // if there is a problem, so let's filter the errors down and pop the first
        // off of the stack to return.
        let errs: Vec<ldt_err::Error> = lines
            .iter()
            .map(|(iline, line)| self.process_line(iline, line))
            .filter_map(|res| {
                if res.is_err() {
                    Some(res.unwrap_err())
                } else {
                    None
                }
            })
            .collect();

        // Check if we have any errors queued. If so, let's return it.
        if !errs.is_empty() {
            return Err(errs.first().unwrap().clone().into());
        }

        Ok(())
    }

    /// Is responsible for processing the lines of the file, and parsing values where necessary.
    pub fn process_line(&mut self, iline: &usize, line: &str) -> Result<(), ldt_err::Error> {
        match *iline {
            // The header row: contains company identification / data bank / version / format identification max.
            1 => {
                self.header = line.to_owned();
                Ok(())
            }
            // Get the type indicator.
            2 => match line.parse::<usize>() {
                Ok(val) => match val.try_into() {
                    Ok(ltype) => {
                        self.ltype = ltype;
                        Ok(())
                    }
                    Err(err) => Err(ldt_err::Error::FromPrimitiveError(*iline, Rc::new(err))),
                },
                Err(err) => Err(ldt_err::Error::ParseIntError(*iline, err)),
            },
            // Get the symmetry indicator.
            3 => match line.parse::<usize>() {
                Ok(val) => match val.try_into() {
                    Ok(sym) => {
                        self.symmetry = sym;
                        Ok(())
                    }
                    Err(err) => Err(ldt_err::Error::FromPrimitiveError(*iline, Rc::new(err))),
                },
                Err(err) => Err(ldt_err::Error::ParseIntError(*iline, err)),
            },
            // Parse C-Planes.
            4 => {
                self.n_cplanes = util::parse_u32(iline, line)? as usize;
                Ok(())
            }
            // Parse C-Plane distance.
            5 => {
                self.cplane_dist = util::parse_f64(iline, line)?;
                Ok(())
            }
            // Parse number of intisities per C-Plane.
            6 => {
                self.n_luminous_intensities_per_cplane = util::parse_u32(iline, line)? as usize;
                Ok(())
            }
            // Parse distance between luminous intensities per cplane.
            7 => {
                self.distance_between_luminous_intensities_per_cplane =
                    util::parse_f64(iline, line)?;
                Ok(())
            }
            // Get the measurement report number.
            8 => {
                self.measurement_report_number = line.to_owned();
                Ok(())
            }
            // Get the name of the lumminaire.
            9 => {
                self.luminaire_name = line.to_owned();
                Ok(())
            }
            // Get the lumminaire number.
            10 => {
                self.luminaire_number = line.to_owned();
                Ok(())
            }
            // Get the filename.
            11 => {
                self.filename = line.to_owned();
                Ok(())
            }
            // Get the date / user that created the file.
            12 => {
                self.date_user = line.to_owned();
                Ok(())
            }
            // Get the length, width, height of the luminaire.
            13 => {
                self.luminaire_length = util::parse_f64(iline, line)?;
                Ok(())
            }
            14 => {
                self.luminaire_width = util::parse_f64(iline, line)?;
                Ok(())
            }
            15 => {
                self.luminaire_height = util::parse_f64(iline, line)?;
                Ok(())
            }
            // Get the dimensions of the luminous area.
            16 => {
                self.luminous_area_length = util::parse_f64(iline, line)?;
                Ok(())
            }
            17 => {
                self.luminous_area_width = util::parse_f64(iline, line)?;
                Ok(())
            }
            18 => {
                self.luminous_area_height_c0 = util::parse_f64(iline, line)?;
                Ok(())
            }
            19 => {
                self.luminous_area_height_c90 = util::parse_f64(iline, line)?;
                Ok(())
            }
            20 => {
                self.luminous_area_height_c180 = util::parse_f64(iline, line)?;
                Ok(())
            }
            21 => {
                self.luminous_area_height_c270 = util::parse_f64(iline, line)?;
                Ok(())
            }
            // Get the downward flux fraction.
            22 => {
                self.downward_flux_fraction = util::parse_f64(iline, line)?;
                Ok(())
            }
            // Get the light output ratio luminaire.
            23 => {
                self.light_output_ratio_luminaire = util::parse_f64(iline, line)?;
                Ok(())
            }
            // Get the luninout intensity conversation factor.
            24 => {
                self.luminous_intensity_conversion_factor = util::parse_f64(iline, line)?;
                Ok(())
            }
            // Get the tilt of the lamp.
            25 => {
                self.tilt = util::parse_f64(iline, line)?;
                Ok(())
            }
            // Get the number of standard lamp sets.
            26 => {
                self.n_lamp_sets = util::parse_u32(iline, line)? as usize;
                Ok(())
            }
            // Get the number of lamps in a set.
            i if self.lamp_section(i, 0) => {
                self.n_lamp.push(util::parse_i32(iline, line)?);
                Ok(())
            }
            // Get the type of lamp.
            i if self.lamp_section(i, 1) => {
                self.lamp_type.push(line.to_owned());
                Ok(())
            }
            // Get the total luminous intensities of the lamps.
            i if self.lamp_section(i, 2) => {
                self.tot_luminous_flux.push(util::parse_f64(iline, line)?);
                Ok(())
            }
            // Get the colour temperature.
            i if self.lamp_section(i, 3) => {
                self.color_temperature.push(line.to_owned());
                Ok(())
            }
            // Get the colour rendering group.
            i if self.lamp_section(i, 4) => {
                self.color_rendering_group.push(line.to_owned());
                Ok(())
            }
            // Get the wattage.
            i if self.lamp_section(i, 5) => {
                self.wattage.push(util::parse_f64(iline, line)?);
                Ok(())
            }
            // Get the direct ratios of room indices.
            i if i >= LAMP_SECTION_START + N_LAMP_PARAMS * self.n_lamp_sets
                && i < LAMP_SECTION_START + N_LAMP_PARAMS * self.n_lamp_sets + 10 =>
            {
                self.direct_ratios.push(util::parse_f64(iline, line)?);
                Ok(())
            }
            // Get the C-angles.
            i if self.is_c_angles(i) => {
                self.c_angles.push(util::parse_f64(iline, line)?);
                Ok(())
            }
            // Get the G-angles.
            i if self.is_g_angles(i) => {
                self.g_angles.push(util::parse_f64(iline, line)?);
                Ok(())
            }
            // Get the luminous intensities.
            i if self.is_luminous_intensities(i) => {
                self.intensities.push(util::parse_f64(iline, line)?);
                Ok(())
            }
            _ => Err(ldt_err::Error::TooManyLines(*iline)),
        }
    }

    /// Helps filter down the lines in the file that correspond to a certain parameter in a lamp set.
    fn lamp_section(&self, iline: usize, isect: usize) -> bool {
        iline >= LAMP_SECTION_START + isect * self.n_lamp_sets
            && iline < LAMP_SECTION_START + (isect + 1) * self.n_lamp_sets
    }

    /// Provides the current index in the current parameter for the lamp set.
    fn i_lamp_section(&self, iline: usize, isect: usize) -> usize {
        iline - (LAMP_SECTION_START + isect * self.n_lamp_sets)
    }

    /// Filters out the lines that contain C-angles.
    fn is_c_angles(&self, iline: usize) -> bool {
        iline >= LAMP_SECTION_START + N_LAMP_PARAMS * self.n_lamp_sets + 10
            && iline < LAMP_SECTION_START + N_LAMP_PARAMS * self.n_lamp_sets + 10 + self.n_cplanes
    }

    /// Provides the current index of the C-angle at this line.
    fn i_c_angle(&self, iline: usize) -> usize {
        iline - (LAMP_SECTION_START + N_LAMP_PARAMS * self.n_lamp_sets + 10)
    }

    /// Filter out the lines that contain G-angles.
    fn is_g_angles(&self, iline: usize) -> bool {
        iline >= LAMP_SECTION_START + N_LAMP_PARAMS * self.n_lamp_sets + 10 + self.n_cplanes
            && iline
                < LAMP_SECTION_START
                    + N_LAMP_PARAMS * self.n_lamp_sets
                    + 10
                    + self.n_cplanes
                    + self.n_luminous_intensities_per_cplane
    }

    /// Provides the current index of the G-angle at this line.
    fn i_g_angle(&self, iline: usize) -> usize {
        iline - (LAMP_SECTION_START + N_LAMP_PARAMS * self.n_lamp_sets + 10 + self.n_cplanes)
    }

    /// Filter out the lines that contain the luminous intensities.
    fn is_luminous_intensities(&self, iline: usize) -> bool {
        iline
            >= LAMP_SECTION_START
                + N_LAMP_PARAMS * self.n_lamp_sets
                + 10
                + self.n_cplanes
                + self.n_luminous_intensities_per_cplane
    }

    /// Filter out the lines that contain the luminous intensities.
    fn i_luminous_intensity(&self, iline: usize) -> usize {
        iline
            - (LAMP_SECTION_START
                + N_LAMP_PARAMS * self.n_lamp_sets
                + 10
                + self.n_cplanes
                + self.n_luminous_intensities_per_cplane)
    }

    /// The Mc1 parameter, as defined by the spec.
    pub fn mc1(&self) -> usize {
        match &self.symmetry {
            EulumdatSymmetry::NoSymmetry => 1,
            EulumdatSymmetry::AboutVerticalAxis => 1,
            EulumdatSymmetry::C0C180Plane => 1,
            EulumdatSymmetry::C90C270Plane => 3 * (self.n_cplanes() / 4) + 1,
            EulumdatSymmetry::C0C180C90C270Plane => 1,
        }
    }

    /// The Mc1 parameter, as defined by the spec.
    pub fn mc2(&self) -> usize {
        match &self.symmetry {
            EulumdatSymmetry::NoSymmetry => self.n_cplanes(),
            EulumdatSymmetry::AboutVerticalAxis => 1,
            EulumdatSymmetry::C0C180Plane => self.n_cplanes() / 2 + 1,
            EulumdatSymmetry::C90C270Plane => self.mc1() + self.n_cplanes / 2,
            EulumdatSymmetry::C0C180C90C270Plane => self.n_cplanes() / 4 + 1,
        }
    }

    /// Get the expected number of lines in the file.
    fn n_file_lines(&self) -> usize {
        LAMP_SECTION_START // The fixed length parameter section of the file. 
        + N_LAMP_PARAMS * self.n_lamp_sets  // The defintion of lamp sets.
        + 10 // The fixed-length (10-long) direct indices section.
        + self.n_cplanes // The number C-plane angles. 
        + self.n_luminous_intensities_per_cplane // The G-plane angles. 
        + (self.mc2() - self.mc1() + 1) * self.n_luminous_intensities_per_cplane()
        // The number of luminous intensities, which is dependent on the symmetry of the object.
    }

    /// Writes the currently loaded EULUMDAT file to a specified file.
    /// The written value is determined by `EulumdatFile::to_string(&self)`.
    pub fn to_file(&self, outpath: &Path) -> Result<(), Error> {
        let mut file = File::create(outpath)?;
        file.write(self.to_string().as_bytes())?;
        Ok(())
    }

    /// Gets the planes from this file.
    pub fn get_planes(&self) -> Vec<Plane> {
        let mut planes: Vec<Plane> = self
            .intensities
            .chunks(self.n_luminous_intensities_per_cplane)
            .zip(self.c_angles.iter())
            .map(|(intens, c_angle)| {
                let mut pl = Plane::new();
                // First set the angle.
                pl.set_angle(degrees_to_radians(*c_angle));
                // Set the angles of the plane from the angles in our C-Plane.
                pl.set_angles(
                    self.g_angles
                        .iter()
                        .map(|ang_deg| degrees_to_radians(*ang_deg))
                        .collect::<Vec<f64>>(),
                );
                // Set the intensities from this chunk of angles.
                pl.set_intensities(Vec::from(intens));
                // Set the flux units of the intensities.
                pl.set_units(IntensityUnits::Candela);
                // Set the units of the units.
                pl.set_orientation(PlaneOrientation::Vertical);

                pl
            })
            .collect();

        // Now fill the planes if we have symmetry.
        // Note that if we have spherical symmetry,
        let mut tmp_planes = planes.clone();

        // Fill the last 180 degrees of the plane from the C0-C180 plane contents.
        if self.symmetry == EulumdatSymmetry::C0C180C90C270Plane {
            tmp_planes.append(
                &mut planes
                    .into_iter()
                    .rev()
                    .skip(1)
                    .map(|mut pl| {
                        *pl.mut_angle() = (FRAC_PI_2 - pl.angle()) + FRAC_PI_2;
                        pl
                    })
                    .collect(),
            );
            planes = tmp_planes.clone();
        }

        // Fill the last 180 degrees of the plane from the C0-C180 plane contents.
        if self.symmetry == EulumdatSymmetry::C0C180Plane
            || self.symmetry == EulumdatSymmetry::C0C180C90C270Plane
        {
            let take_planes = planes.iter().count() - 2;
            tmp_planes.append(
                &mut planes
                    .into_iter()
                    .rev()
                    .skip(1)
                    .take(take_planes)
                    .map(|mut pl| {
                        *pl.mut_angle() = (PI - pl.angle()) + PI;
                        pl
                    })
                    .collect(),
            );
            planes = tmp_planes.clone();
        }

        // On the other hand, let's generate the symmetry for the C90-C270 case.
        if self.symmetry == EulumdatSymmetry::C90C270Plane {
            let half = planes.iter().count() / 2;
            // Assemble the first half (0 - 90)
            tmp_planes = planes
                .iter()
                .skip(1)
                .take(half)
                .rev()
                .map(|pl| {
                    let mut new_plane = pl.clone();
                    *new_plane.mut_angle() = PI - pl.angle();
                    new_plane
                })
                .collect();
            // Now copy the C90 - C270 planes.
            tmp_planes.append(&mut planes.clone());
            // Now merge the C270 - C0 planes onto the end.
            tmp_planes.append(
                &mut planes
                    .into_iter()
                    .skip(half + 1)
                    .take(half - 1)
                    .rev()
                    .map(|mut pl| {
                        *pl.mut_angle() = pl.angle() + 2.0 * (3.0 * FRAC_PI_2 - pl.angle());
                        pl
                    })
                    .collect(),
            );
            planes = tmp_planes;
        }

        planes
    }
}

impl ToString for EulumdatFile {
    /// Writes the object to a EULUMDAT format string, which can be written to a file.
    /// We need to be careful that we limit to the correct size of string, as defined by the spec.
    fn to_string(&self) -> String {
        (1..self.n_file_lines())
            .into_iter()
            .fold("".to_string(), |accum, iline| {
                accum
                    + format!(
                        "{}\n",
                        match iline {
                            1 => self.header.clone(),
                            2 => (self.ltype.clone() as usize).to_string(),
                            3 => (self.symmetry.clone() as usize).to_string(),
                            4 => self.n_cplanes.to_string(),
                            5 => self.cplane_dist.to_string(),
                            6 => self.n_luminous_intensities_per_cplane.to_string(),
                            7 => self
                                .distance_between_luminous_intensities_per_cplane
                                .to_string(),
                            8 => self.measurement_report_number.clone(),
                            9 => self.luminaire_name.clone(),
                            10 => self.luminaire_number.clone(),
                            11 => self.filename.clone(),
                            12 => self.date_user.clone(),
                            13 => self.luminaire_length.to_string(),
                            14 => self.luminaire_width.to_string(),
                            15 => self.luminaire_height.to_string(),
                            16 => self.luminous_area_length.to_string(),
                            17 => self.luminous_area_width.to_string(),
                            18 => self.luminous_area_height_c0.to_string(),
                            19 => self.luminous_area_height_c90.to_string(),
                            20 => self.luminous_area_height_c180.to_string(),
                            21 => self.luminous_area_height_c270.to_string(),
                            22 => self.downward_flux_fraction.to_string(),
                            23 => self.light_output_ratio_luminaire.to_string(),
                            24 => self.luminous_intensity_conversion_factor.to_string(),
                            25 => self.tilt.to_string(),
                            26 => self.n_lamp_sets.to_string(),
                            i if self.lamp_section(i, 0) => {
                                self.n_lamp[self.i_lamp_section(iline, 0)].to_string()
                            }
                            i if self.lamp_section(i, 1) => {
                                self.lamp_type[self.i_lamp_section(iline, 1)].clone()
                            }
                            i if self.lamp_section(i, 2) => {
                                self.tot_luminous_flux[self.i_lamp_section(iline, 2)].to_string()
                            }
                            i if self.lamp_section(i, 3) => {
                                self.color_temperature[self.i_lamp_section(iline, 3)].clone()
                            }
                            i if self.lamp_section(i, 4) => {
                                self.color_rendering_group[self.i_lamp_section(iline, 4)].clone()
                            }
                            i if self.lamp_section(i, 5) => {
                                self.wattage[self.i_lamp_section(iline, 5)].to_string()
                            }
                            i if i >= LAMP_SECTION_START + N_LAMP_PARAMS * self.n_lamp_sets
                                && i < LAMP_SECTION_START
                                    + N_LAMP_PARAMS * self.n_lamp_sets
                                    + 10 =>
                            {
                                let i_direct_ratio =
                                    iline - (LAMP_SECTION_START + N_LAMP_PARAMS * self.n_lamp_sets);
                                self.direct_ratios[i_direct_ratio].to_string()
                            }
                            i if self.is_c_angles(i) => {
                                self.c_angles[self.i_c_angle(iline)].to_string()
                            }
                            i if self.is_g_angles(i) => {
                                self.g_angles[self.i_g_angle(iline)].to_string()
                            }
                            i if self.is_luminous_intensities(i) => {
                                self.intensities[self.i_luminous_intensity(iline)].to_string()
                            }
                            _ => "".to_owned(),
                        }
                    )
                    .as_ref()
            })
    }
}

impl From<EulumdatFile> for PhotometricWeb {
    fn from(eul: EulumdatFile) -> Self {
        let mut photweb = PhotometricWeb::new();
        // Get the angles.
        photweb.set_planes(eul.get_planes());
        photweb
    }
}

//TODO: Implement conversion.
impl PhotometricWebReader for EulumdatFile {
    fn read(&self, path: &Path) -> Result<PhotometricWeb, Error> {
        let eul_file = Self::parse_file(path)?;
        let photweb: PhotometricWeb = eul_file.into();
        Ok(photweb)
    }
}
