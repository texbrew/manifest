use error::Error;
use serde_yaml;
use std::fs;
use std::path::{Path, PathBuf};

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
    pub url_base: Option<String>,
    pub items: Vec<SvnItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "svn")]
    pub svn_group: SvnGroup,
}

impl File {
    pub fn new(path: &Path) -> Result<File, Error> {
        log::debug!("Opening path: {:?}", path);
        let file = fs::File::open(path)?;
        Ok(serde_yaml::from_reader(file)?)
    }
}
