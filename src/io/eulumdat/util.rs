use super::err as ldt_err;

// Attempts to parse an int value and passes back an appropriate error if unable.
pub fn parse_u32(iline: &usize, line: &str) -> Result<u32, ldt_err::Error> {
    match line.parse() {
        Ok(val) => Ok(val),
        Err(err) => Err(ldt_err::Error::ParseIntError(*iline, err)),
    }
}

// Attempts to parse a signed int value and passes back an appropriate error if unable.
pub fn parse_i32(iline: &usize, line: &str) -> Result<i32, ldt_err::Error> {
    match line.parse() {
        Ok(val) => Ok(val),
        Err(err) => Err(ldt_err::Error::ParseIntError(*iline, err)),
    }
}

// Attempts to parse an float value and passes back an appropriate error if unable.
pub fn parse_f64(iline: &usize, line: &str) -> Result<f64, ldt_err::Error> {
    match line.parse() {
        Ok(val) => Ok(val),
        Err(err) => Err(ldt_err::Error::ParseFloatError(*iline, err)),
    }
}
