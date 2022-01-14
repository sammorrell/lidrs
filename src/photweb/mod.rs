//! Photometric Web
//!
//! A module that contains a common object for representing luminous intensity distributions via photometric webs.

// Basic traits.
pub mod photweb_reader;
pub mod photweb_writer;
pub use self::{photweb_reader::*, photweb_writer::*};

pub mod photweb;
pub mod plane;
pub mod units;

// Builders
pub mod photweb_builder;

pub use self::{photweb::*, photweb_builder::*, plane::*, units::*};

#[cfg(test)]
mod tests;
