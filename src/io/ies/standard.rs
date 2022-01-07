/// A list of possible IES standards.
#[derive(Debug, PartialEq, Eq)]
pub enum IesStandard {
    Lm63_1986,
    Lm63_1991,
    Lm63_1995,
    Lm63_2002,
}

impl Default for IesStandard {
    fn default() -> Self {
        Self::Lm63_1986
    }
}

impl From<&str> for IesStandard {
    /// This converts the header string of the file into a known standard.
    /// The default case will catch anything that
    fn from(str: &str) -> Self {
        match str {
            "IESNA91" => IesStandard::Lm63_1991,
            "IESNA:LM-63-1995" => IesStandard::Lm63_1995,
            "IESNA:LM-63-2002" => IesStandard::Lm63_2002,
            _ => IesStandard::Lm63_1986,
        }
    }
}

impl From<String> for IesStandard {
    fn from(str: String) -> Self {
        Self::from(str.as_str())
    }
}
