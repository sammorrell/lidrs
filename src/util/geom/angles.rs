use std::f64::consts::PI;

#[inline]
#[must_use]
/// Converts a given angle in degrees into radians.
pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * (PI / 180.0)
}

#[inline]
#[must_use]
/// Converts a given angle in radians into degrees.
pub fn radians_to_degrees(radian: f64) -> f64 {
    radian * (180.0 / PI)
}

#[inline]
#[must_use]
/// Gets the difference (in radians) between two angles.
pub fn angle_difference(angle1: f64, angle2:f64) -> f64 {
    let dot_prod = angle1.cos().mul_add(angle2.cos(), angle1.sin() * angle2.sin());
    let mag = (angle1.cos().powi(2) + angle1.sin().powi(2)).sqrt() * (angle2.cos().powi(2) + angle2.sin().powi(2)).sqrt();
    (dot_prod / mag).acos()
}

#[cfg(test)]
mod tests {
    use super::{degrees_to_radians, radians_to_degrees, angle_difference};
    use std::f64::consts::PI;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_degrees_to_radians() {
        assert_eq!(degrees_to_radians(0.0), 0.0);
        assert_eq!(degrees_to_radians(90.0), PI / 2.0);
        assert_eq!(degrees_to_radians(180.0), PI);
        assert_eq!(degrees_to_radians(360.0), 2.0 * PI);
    }

    #[test]
    fn test_radians_to_degrees() {
        assert_eq!(radians_to_degrees(0.0), 0.0);
        assert_eq!(radians_to_degrees(PI / 2.0), 90.0);
        assert_eq!(radians_to_degrees(PI), 180.0);
        assert_eq!(radians_to_degrees(2.0 * PI), 360.0);
    }

    #[test]
    fn test_angle_difference() {
        assert_abs_diff_eq!(angle_difference(0., PI), PI, epsilon = 1E-6);
        assert_abs_diff_eq!(angle_difference(PI, 0.), PI, epsilon = 1E-6);
        assert_abs_diff_eq!(angle_difference(PI, 2.0 * PI), PI, epsilon = 1E-6);
        // Now try some cases where we are around the 0 / 2PI point to check that we can handle this. 
        assert_abs_diff_eq!(angle_difference(degrees_to_radians(350.), degrees_to_radians(10.)), degrees_to_radians(20.), epsilon = 1E-6);
        assert_abs_diff_eq!(angle_difference(degrees_to_radians(10.), degrees_to_radians(350.)), degrees_to_radians(20.), epsilon = 1E-6);
    }
}
