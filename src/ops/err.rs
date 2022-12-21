use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    NoPlanes,
    InconsistentNumberOfPlanes(usize, usize, usize),
    InconsistentIntensitiesInPlane(usize, usize),
    InconsistentPlaneAngles,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                Self::NoPlanes => format!("No photometric planes found in web.. "),
                Self::InconsistentNumberOfPlanes(ref expect, ref found, ref idx) => format!("Expected {} planes. Found {} planes and index {}. ", expect, found, idx),
                Self::InconsistentIntensitiesInPlane(ref expect, ref found) => format!("Expected {} intensities in plane. Found {} intensisites. ", expect, found),
                Self::InconsistentPlaneAngles => format!("Angles are inconsistent between photometric web planes. "),
            }
        })
    }
}
