use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum LdtSymmetry {
    NoSymmetry = 0,
    AboutVerticalAxis = 1,
    C0C180Plane = 2,    
    C90C270Plane = 3,
    C0C180C90C270Plane = 4,
}

impl Default for LdtSymmetry {
    fn default() -> Self {
        LdtSymmetry::NoSymmetry
    }
}