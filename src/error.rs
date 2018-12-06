use std::fmt::{Display, Formatter, Result};
use std::io;

#[derive(Debug)]
pub enum Error {
    WhichError(String, which::Error),
    IOError(io::Error),
    YamlError(serde_yaml::Error),
    UrlParseError(url::ParseError),
    StringError(String),
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

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Error {
        UrlParseError(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Error {
        StringError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            WhichError(name, e) => write!(f, "Error finding the '{}' command: {}", name, e),
            IOError(e) => write!(f, "I/O error: {}", e),
            YamlError(e) => write!(f, "YAML error: {}", e),
            UrlParseError(e) => write!(f, "URL parse error: {}", e),
            StringError(e) => write!(f, "{}", e),
        }
    }
}
