use std::fmt::{Display, Formatter, Result};
use std::io;

#[derive(Debug)]
pub enum Error {
    WhichError(String, which::Error),
    IOError(io::Error),
    YamlError(serde_yaml::Error),
}

use Error::*;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        IOError(e)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Error {
        YamlError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            WhichError(name, e) => write!(f, "Error finding the '{}' command: {}", name, e),
            IOError(e) => write!(f, "I/O error: {}", e),
            YamlError(e) => write!(f, "YAML error: {}", e),
        }
    }
}
