use crate::{err::Error, photweb::PhotometricWeb};
use std::path::Path;

/// A trait that can read from a file.
pub trait PhotometricWebReader {
    fn read(&self, path: &Path) -> Result<PhotometricWeb, Error>;
}
