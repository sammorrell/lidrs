use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum LdtType {
    PointSourceWithSymmetryAboutVerticalAxis = 1,
    LinearLumminaire = 2,
    PointSourceWithOtherSymmetry = 3,
}

impl Default for LdtType {
    fn default() -> Self {
        LdtType::PointSourceWithSymmetryAboutVerticalAxis
    }
}