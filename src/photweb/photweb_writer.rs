use crate::{err::Error, photweb::PhotometricWeb};
use std::path::Path;

/// A trait that can write to a file.
pub trait PhotometricWebWriter {
    fn write(photweb: &PhotometricWeb, path: &Path) -> Result<(), Error>;
}
