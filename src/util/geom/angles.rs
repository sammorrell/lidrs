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

#[cfg(test)]
mod tests {
    use super::{degrees_to_radians, radians_to_degrees};
    use std::f64::consts::PI;

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
}
