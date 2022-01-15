use property::Property;
use std::default::Default;
use crate::util::geom::radians_to_degrees;

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
    /// The angle of the plane, stored in radians.
    angle: f64,
    /// The orientation of the plane. 
    orientation: PlaneOrientation,
    /// A vector containing angles within the plane, stored in radians
    angles: Vec<f64>,
    /// A vector containing intensities, stored in the units indicated in the `units` field.
    intensities: Vec<f64>,
    /// The units in which the luminous intensities are stored. 
    units: IntensityUnits,
}

impl Plane {
    // Create a new Plane object, filled with default values.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Get the angle of the plane in degrees. 
    pub fn angle_deg(&self) -> f64 {
        radians_to_degrees(self.angle)
    }

    /// Get the internal angles in the object as degrees.
    pub fn angles_deg(&self) -> Vec<f64> {
        self.angles
            .iter()
            .map(|angle_radians| radians_to_degrees(*angle_radians))
            .collect()
    }
}

#[cfg(test)]
mod tests {

}