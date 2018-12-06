use error::Error;
use std::path::PathBuf;

/// A wrapper for which::which that adapts the error.
pub fn which(name: String) -> Result<PathBuf, Error> {
    which::which(&name).map_err(|e| Error::WhichError(name, e))
}
