use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum EulumdatType {
    PointSourceWithSymmetryAboutVerticalAxis = 1,
    LinearLumminaire = 2,
    PointSourceWithOtherSymmetry = 3,
}

impl Default for EulumdatType {
    fn default() -> Self {
        EulumdatType::PointSourceWithSymmetryAboutVerticalAxis
    }
}
