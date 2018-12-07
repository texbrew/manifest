use crate::error::Error;
use std::path::PathBuf;

/// A wrapper for which::which that adapts the error.
pub fn which(name: &str) -> Result<PathBuf, Error> {
    which::which(name).map_err(|e| Error::WhichError(String::from(name), e))
}
