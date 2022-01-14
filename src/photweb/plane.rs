use property::Property;
use std::default::Default;

use super::units::IntensityUnits;

#[derive(Debug, Clone)]
pub enum PlaneOrientation {
    Vertical,
    Horizontal,
}

impl Default for PlaneOrientation {
    fn default() -> Self {
        PlaneOrientation::Vertical
    }
}

#[derive(Debug, Clone, Default, Property)]
pub struct Plane {
    angle: f64,
    orientation: PlaneOrientation,
    angles: Vec<f64>,
    intensities: Vec<f64>,
    units: IntensityUnits,
}

impl Plane {
    // Create a new Plane object, filled with default values.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
