use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, TryFromPrimitive, PartialEq)]
#[repr(usize)]
pub enum IesPhotometryType {
    TypeC = 1,
    TypeB = 2,
    TypeA = 3,
}

impl Default for IesPhotometryType {
    /// I will return Type C by default, as it permits the most leway.
    fn default() -> Self {
        IesPhotometryType::TypeC
    }
}
