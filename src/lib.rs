//! Lidrs - A Rust Crate for Reading, Writing and Working with Luminous Intensity Distributions.
//!
//! A fully-rust crate for reading and writing luminous intensity distributions / photometric files.
//! There are currently implemented parsers for:
//! - EULUMDAT (.ldt / .eul) formatted ASCII files.
//! - Illuminating Engineering Society (.ies) formatted ASCII files.
//! 
//! It is also able to interpret these files to produce a full spherical photometric web from the inputs.
//! 

pub mod err;
pub mod io;
pub mod photweb;
pub mod util;
