//! EULUMTDAT File Struct.
//!
//! This module contains the structs and enums for supporting parsing of the EULUMDAT (LDT) file format.
//! Although I was unable to find any formal documentation during the development, my information on this format was drawn from two different sources:
//! - Unofficial EULUMDAT file format specification: <http://www.helios32.com/Eulumdat.htm>
//! - The KeyLights Appendix B file specification (courtesy of Transoft solutions): <https://keysofthelp.transoftsolutions.com/KeyLIGHTS/6.3/Appendix%20B%20EULUMDAT%20File%20Format.htm>
//! - Documentation by Paul Bourke: <http://paulbourke.net/dataformats/ldt/>

pub mod err;
pub mod eulumdat_file;
pub mod ltyp;
pub mod symmetry;
pub mod util;
pub use self::{err::*, eulumdat_file::*, ltyp::*, symmetry::*, util::*};

#[cfg(test)]
pub mod tests;
