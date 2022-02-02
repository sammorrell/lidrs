use num_enum::TryFromPrimitive;

/// Determine the type of photometry contained within the file.
/// A brief description of each type can be found on Owen Ransens web page here:  <http://www.ransen.com/photometric/Type-A-Type-B-Type-C-IES-IESNA-Photometry-Files.htm>
#[derive(Debug, Clone, TryFromPrimitive, PartialEq)]
#[repr(usize)]
pub enum IesPhotometryType {
    /// This type is generally used for internal lighting and use the C-Gamma system for measurements. 
    /// This type of file is by far the most popular. In this system, gamma = 0 corresponds to downwards.
    /// The C-angle is the anti-clockwise angle, around the gamma = 0 axis. 
    TypeC = 1,
    /// This type is used for floodlights, and the photometry is expressed in a VH system.
    /// In this system, the lights are expressed as a hemisphere of intensities.
    TypeB = 2,
    /// This type is used for car headlights. 
    /// In this case, the polar axis corresponds to the major axis of the luminaire, i.e. the direction that the light is pointing. 
    TypeA = 3,
}

impl Default for IesPhotometryType {
    /// I will return Type C by default, as it permits the most leway.
    fn default() -> Self {
        IesPhotometryType::TypeC
    }
}
