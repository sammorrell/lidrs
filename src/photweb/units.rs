#[derive(Debug, Clone)]
pub enum IntensityUnits {
    Candela,
}

impl Default for IntensityUnits {
    fn default() -> Self {
        IntensityUnits::Candela
    }
}
