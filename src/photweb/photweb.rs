use super::Plane;
use property::Property;
use std::{default::Default, f64::consts::PI};

#[derive(Default, Debug, Property)]
#[property(get(public), set(public))]
pub struct PhotometricWeb {
    /// The planes that are contained in the photometric web.
    /// Note: if we have a single element in this vector, if is assumed spherically symmetric.
    planes: Vec<Plane>,
}

impl PhotometricWeb {
    /// Returns a new instance of the PhotometricWeb object with default values.
    pub fn new() -> PhotometricWeb {
        Self {
            ..Default::default()
        }
    }

    /// Returns the number of planes in the photometric web.
    pub fn n_planes(&self) -> usize {
        self.planes.iter().count()
    }

    /// Is this a full web, or spherically symmetric?
    pub fn is_spherically_symmetric(&self) -> bool {
        self.planes.iter().count() == 1
    }

    /// The dphi for a given plane - used during integration.
    pub fn delta_angle(&self, i: usize) -> f64 {
        if self.is_spherically_symmetric() {
            2.0 * PI
        } else {
            match i {
                0 => self.planes[1].angle() - self.planes[0].angle(),
                x if x >= self.planes.iter().count() - 1 => {
                    self.planes[i].angle() - self.planes[i - 1].angle()
                }
                _ => {
                    0.5 * ((self.planes[i].angle() - self.planes[i - 1].angle())
                        + (self.planes[i + 1].angle() - self.planes[i].angle()))
                }
            }
        }
    }

    /// Integrates the total energy coming from the intensity distribution.
    /// This makes use of the integration that is a part of the planes.
    pub fn total_intensity(&self) -> f64 {
        self.planes
            .iter()
            .enumerate()
            .map(|(i, p)| p.integrate_intensity() * self.delta_angle(i))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::{PhotometricWeb, Plane};
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    /// Check that by default, our integrated intensity is zero.
    #[test]
    fn test_integrate_zero() {
        let web = PhotometricWeb::new();
        let res = web.total_intensity();
        assert_eq!(res, 0.0);
    }

    /// In this case, I am filling the array with a constant of 1.0, which simplifies the integral for each plane
    /// to being $\int^{\pi}_{0} \sin(\phi) d\phi$. Once fully integrated and substituted, this will result in a final
    /// value of 2.0 per plane. As I am integrating the whole sphere, we will end up with $4 \pi$ as the output.
    /// We need to account for numerical error, so I will see how low I can go with the tolerance.
    #[test]
    fn test_integrate_spherically_symmetric() {
        let mut plane = Plane::new();
        plane.set_angle(0.0);
        plane.set_angles_degrees(
            &(0..181)
                .into_iter()
                .map(|ang_i| ang_i as f64)
                .collect::<Vec<f64>>(),
        );
        plane.set_intensities(plane.angles().iter().map(|_| 1.0).collect::<Vec<f64>>());

        let mut web = PhotometricWeb::new();
        web.set_planes(vec![plane]);

        // We should be integrating by
        let int = web.total_intensity();

        // Check that this is true to within 0.01 per cent.
        assert_abs_diff_eq!(int, 4.0 * PI, epsilon = (4.0 * PI) * 1E-4);
    }

    /// This the same as the `test_integrate_spherically_symmetric` test case, however with a series of planes as opposed to spherical symmetry.
    /// I am filling the array with a constant of 1.0, which simplifies the integral for each plane
    /// to being $\int^{\pi}_{0} \sin(\phi) d\phi$. Once fully integrated and substituted, this will result in a final
    /// value of 2.0 per plane. As I am integrating the whole sphere, we will end up with $4 \pi$ as the output.
    /// We need to account for numerical error, so I will see how low I can go with the tolerance.
    #[test]
    fn test_integrate_planes() {
        let mut plane = Plane::new();
        plane.set_angle(0.0);
        plane.set_angles_degrees(
            &(0..181)
                .into_iter()
                .map(|ang_i| ang_i as f64)
                .collect::<Vec<f64>>(),
        );
        plane.set_intensities(plane.angles().iter().map(|_| 1.0).collect::<Vec<f64>>());

        let mut web = PhotometricWeb::new();
        web.set_planes(
            (0..360)
                .step_by(10)
                .into_iter()
                .map(|ang_deg| {
                    let mut new_plane = plane.clone();
                    new_plane.set_angle_degrees(ang_deg as f64);
                    new_plane
                })
                .collect::<Vec<Plane>>(),
        );

        // We should be integrating by
        let int = web.total_intensity();

        // Check that this is true to within 0.01 per cent.
        assert_abs_diff_eq!(int, 4.0 * PI, epsilon = (4.0 * PI) * 1E-4);
    }
}
