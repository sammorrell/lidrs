use crate::util::geom::{degrees_to_radians, radians_to_degrees};
use property::Property;
use std::{default::Default};

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
#[property(get(public), set(public))]
pub struct Plane {
    /// The angle of the plane, stored in radians.
    angle: f64,
    /// The width of the plane, in radians. 
    width: f64,
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

    /// Sets the angle of the plane, given in degrees.
    pub fn set_angle_degrees(&mut self, ang_deg: f64) {
        self.set_angle(degrees_to_radians(ang_deg));
    }

    /// Sets the angles, given in degrees.
    pub fn set_angles_degrees(&mut self, ang_deg: &Vec<f64>) {
        self.set_angles(
            ang_deg
                .iter()
                .map(|ang| degrees_to_radians(*ang))
                .collect::<Vec<f64>>(),
        );
    }

    /// Returns the number of angle / intensity pairs in the current plane object.
    pub fn n_samples(&self) -> usize {
        self.angles.iter().count()
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

    /// The delta angle for a given angle in the plane - used for integration.
    pub fn delta_angle(&self, i: usize) -> f64 {
        match i {
            0 => self.angles[1] - self.angles[0],
            x if x >= self.angles.iter().count() - 1 => self.angles[i] - self.angles[i - 1],
            _ => {
                0.5 * ((self.angles[i] - self.angles[i - 1])
                    + (self.angles[i + 1] - self.angles[i]))
            }
        }
    }

    /// Integrate the total energy being emitted by this plane.
    pub fn integrate_intensity(&self) -> f64 {
        self.width * self.intensities
            .iter()
            .enumerate()
            .map(|(i, int)| int * f64::sin(self.angles[i]) * self.delta_angle(i))
            .sum::<f64>()
    }
}

#[cfg(test)]
mod tests {
    use super::Plane;
    use approx::assert_abs_diff_eq;

    /// In this case, I am filling the array with a constant of 1.0, which simplifies the integral for each plane
    /// to being $\int^{\pi}_{0} \sin(\phi) d\phi$. Once fully integrated and substituted, this will result in a final
    /// value of 2.0. We need to account for numerical error, so I will see how low I can go with the tolerance.
    #[test]
    fn test_integrate_plane() {
        let mut plane = Plane::new();
        plane.set_angle(0.0);
        plane.set_width(1.0);
        plane.set_angles_degrees(
            &(0..181)
                .into_iter()
                .map(|ang_i| ang_i as f64)
                .collect::<Vec<f64>>(),
        );
        plane.set_intensities(plane.angles().iter().map(|_| 1.0).collect::<Vec<f64>>());

        // Check that this is true to within 0.01 per cent.
        assert_abs_diff_eq!(plane.integrate_intensity(), 2.0, epsilon = 2.0E-4);
    }
}
