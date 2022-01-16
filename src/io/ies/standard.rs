/// A list of possible IES standards.
#[derive(Debug, PartialEq, Eq)]
pub enum IesStandard {
    Iesna1986,
    Iesna1991,
    Iesna1995,
    Iesna2002,
}

impl Default for IesStandard {
    fn default() -> Self {
        Self::Iesna1986
    }
}

impl From<&str> for IesStandard {
    /// This converts the header string of the file into a known standard.
    /// The default case will catch anything that
    fn from(str: &str) -> Self {
        match str {
            "IESNA91" => IesStandard::Iesna1991,
            "IESNA:LM-63-1995" => IesStandard::Iesna1995,
            "IESNA:LM-63-2002" => IesStandard::Iesna2002,
            _ => IesStandard::Iesna1986,
        }
    }
}

impl From<String> for IesStandard {
    fn from(str: String) -> Self {
        Self::from(str.as_str())
    }
}

impl std::fmt::Display for IesStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IesStandard::Iesna1986 => "",
                IesStandard::Iesna1991 => "IESNA91",
                IesStandard::Iesna1995 => "IESNA:LM-63-1995",
                IesStandard::Iesna2002 => "IESNA:LM-63-2002",
            }
            .to_string()
        )
    }
}
