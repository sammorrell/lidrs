use super::{Error, DELIMITERS_PATTERN};
use regex::Regex;

#[allow(dead_code)]
pub fn parse_float_array(str: &str) -> Result<Vec<f32>, Error> {
    let split_regex = Regex::new(DELIMITERS_PATTERN).unwrap();

    let (vals, errs): (Vec<_>, Vec<_>) = split_regex
        .split(str)
        .map(|str| str.parse::<f32>())
        .partition(Result::is_ok);

    let numbers: Vec<_> = vals.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();

    match errors.first() {
        None => Ok(numbers),
        Some(e) => Err(crate::io::ies::Error::ParseFloatError(0, None, e.clone())),
    }
}
