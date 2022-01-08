use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum EulumdatSymmetry {
    NoSymmetry = 0,
    AboutVerticalAxis = 1,
    C0C180Plane = 2,    
    C90C270Plane = 3,
    C0C180C90C270Plane = 4,
}

impl Default for EulumdatSymmetry {
    fn default() -> Self {
        EulumdatSymmetry::NoSymmetry
    }
}