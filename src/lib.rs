extern crate duct;
extern crate log;
extern crate stderrlog;
extern crate structopt;
extern crate tempfile;
extern crate which as libwhich;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate url;
extern crate walkdir;

mod error;
mod file;
mod gitignore;
mod opt;
mod svn;
mod svnadmin;
mod which;

use error::Error;
use file::File;
use opt::Opt;
use std::path::Path;
use std::path::PathBuf;
use svn::Svn;

pub fn run() -> Result<(), Error> {
    let opt = Opt::new()?;
    let file = File::new(Path::new("manifest.yml"))?;
    log::debug!("File: {:?}", file);
    let svn = Svn::new()?;
    for file::SvnItem {
        url,
        rev,
        path,
        gitignore: _,
    } in file.svn_group.items
    {
        let url = match file.svn_group.url_base.to_owned() {
            None => url,
            Some(url_base) => url_base + &url,
        };
        svn.checkout(
            opt.quiet,
            rev.or(file.svn_group.rev),
            &url,
            path.as_ref().map(PathBuf::as_path),
        )?
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fmt::{self, Display, Formatter};
    use std::io;
    use std::path;
    use svnadmin::SvnAdmin;
    use tempfile::{tempdir, Builder};
    use walkdir::{DirEntry, WalkDir};

    #[derive(Debug)]
    pub enum TestError {
        LibError(super::Error),
        WalkDirError(walkdir::Error),
        StripPrefixError(path::StripPrefixError),
    }

    use self::TestError::*;

    impl From<super::Error> for TestError {
        fn from(e: super::Error) -> TestError {
            LibError(e)
        }
    }

    impl From<io::Error> for TestError {
        fn from(e: io::Error) -> TestError {
            LibError(super::Error::from(e))
        }
    }

    impl From<walkdir::Error> for TestError {
        fn from(e: walkdir::Error) -> TestError {
            WalkDirError(e)
        }
    }

    impl From<path::StripPrefixError> for TestError {
        fn from(e: path::StripPrefixError) -> TestError {
            StripPrefixError(e)
        }
    }

    impl Display for TestError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self {
                LibError(e) => write!(f, "Lib error: {}", e),
                WalkDirError(e) => write!(f, "WalkDir error: {}", e),
                StripPrefixError(e) => write!(f, "StripPrefixError: {}", e),
            }
        }
    }

    fn is_svn_dir(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s == ".svn")
            .unwrap_or(false)
    }

    fn collect_dir_entries(path: &Path) -> Result<Vec<String>, TestError> {
        let mut entries = Vec::new();

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_entry(|e| !is_svn_dir(e))
        {
            // Check for files within the directory. First, strip the directory prefix. Then,
            // since the directory itself will be an empty string, filter it before pushing.
            let entry = format!("{}", entry?.path().strip_prefix(path)?.display());
            if !entry.is_empty() {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    #[test]
    fn svn_create_add_commit_checkout() -> Result<(), TestError> {
        let repo_dir = tempdir()?;
        SvnAdmin::new()?.create(repo_dir.path())?;
        let repo_url = format!("file://{}", repo_dir.path().display());

        let svn = Svn::new()?;

        let entries1 = {
            let checkout_dir = tempdir()?;
            svn.checkout(false, None, &repo_url, Some(checkout_dir.path()))?;

            let file = Builder::new()
                .rand_bytes(10)
                .tempfile_in(checkout_dir.path())?;

            let pwd = env::current_dir()?;
            env::set_current_dir(checkout_dir.path())?;

            svn.add(false, vec![file.path()])?;
            svn.commit(false, "new file", Vec::new())?;

            env::set_current_dir(pwd)?;

            collect_dir_entries(checkout_dir.path())?
        };

        let entries2 = {
            let checkout_dir = tempdir()?;
            svn.checkout(false, None, &repo_url, Some(checkout_dir.path()))?;
            collect_dir_entries(checkout_dir.path())?
        };

        assert_eq!(entries1, entries2);

        Ok(())
    }
}
