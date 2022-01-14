use super::Plane;
use property::Property;
use std::default::Default;

#[derive(Default, Debug, Property)]
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
}
