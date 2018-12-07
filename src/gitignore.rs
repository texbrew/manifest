use crate::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// A pair of a directory path and a vector of directory or file paths within the first directory
// path.
#[derive(Clone, Debug)]
pub struct SubPaths(pub String, pub Vec<String>);

#[derive(Clone, Debug)]
pub struct GitIgnore {
    pub exclude_all: Vec<String>,
    pub exclude_paths: Vec<SubPaths>,
    pub include_paths: Vec<SubPaths>,
}

impl GitIgnore {
    pub fn to_file(&self, dir: &Path) -> Result<(), Error> {
        if !dir.is_dir() {
            return Err(Error::from(format!(
                ".gitignore path is not a directory: {}",
                dir.display()
            )));
        }
        let mut file = File::create(dir.with_file_name(".gitignore"))?;
        for line in &self.exclude_all {
            writeln!(file, "{}", &line)?;
        }
        for SubPaths(dir, paths) in &self.exclude_paths {
            for path in paths {
                writeln!(file, "/{}/{}", &dir, &path)?;
            }
        }
        for SubPaths(dir, paths) in &self.include_paths {
            writeln!(file, "/{}/*", &dir)?;
            for path in paths {
                writeln!(file, "!/{}/{}", &dir, &path)?;
            }
        }
        Ok(())
    }
}
