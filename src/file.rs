use crate::error::Error;
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PathGitIgnore {
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub include: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SvnItem {
    pub url: String,
    pub rev: Option<usize>,
    pub path: Option<PathBuf>,
    pub gitignore: Option<PathGitIgnore>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SvnGroup {
    pub rev: Option<usize>,
    #[serde(with = "url_serde")]
    pub url_base: Option<Url>,
    pub items: Vec<SvnItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(default)]
    pub gitignore: Vec<String>,
    #[serde(rename = "svn")]
    pub svn_group: SvnGroup,
}

impl File {
    pub fn parse(path: &Path) -> Result<File, Error> {
        log::debug!("Opening path: {:?}", path);
        let file = fs::File::open(path)?;
        Ok(serde_yaml::from_reader(file)?)
    }
}
