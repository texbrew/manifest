mod error;
mod file;
mod gitignore;
mod opt;
mod svn;
mod svnadmin;
mod which;

use crate::error::Error;
use crate::file::File;
use crate::gitignore::{GitIgnore, SubPaths};
use crate::opt::Opt;
use crate::svn::Svn;
use std::path::Path;
use std::path::PathBuf;
use url::Url;

pub fn run() -> Result<(), Error> {
    let opt = Opt::init()?;
    let file = File::parse(Path::new("manifest.yml"))?;
    log::debug!("File: {:?}", file);

    let svn = Svn::cmd()?;

    let exclude_all = file.gitignore.to_owned();
    let mut exclude_paths = Vec::new();
    let mut include_paths = Vec::new();

    for file::SvnItem {
        url,
        rev,
        path,
        gitignore,
    } in file.svn_group.items
    {
        // Extract the complete URL from the optional url_base and required SvnItem::url.
        let url = match &file.svn_group.url_base {
            None => Url::parse(&url)?,
            Some(url_base) if url_base.cannot_be_a_base() => {
                return Err(Error::from(format!(
                    "url_base is not a URL base: {}",
                    &url_base
                )))
            }
            Some(url_base) => url_base.join(&url)?,
        };

        let dir: &str = match &path {
            Some(path) => path
                .to_str()
                .ok_or_else(|| format!("Path is not valid UTF-8: {}", url.as_str()))?,
            None => {
                let dir = url
                    .path_segments()
                    .ok_or_else(|| format!("URL has no path: {}", url.as_str()))?
                    .last()
                    .unwrap();
                if dir.is_empty() {
                    return Err(Error::from(format!("URL missing a path: {}", url.as_str())));
                }
                dir
            }
        };
        if let Some(gitignore) = gitignore {
            exclude_paths.push(SubPaths(dir.to_string(), gitignore.exclude));
            include_paths.push(SubPaths(dir.to_string(), gitignore.include));
        }

        svn.checkout(
            opt.quiet,
            rev.or(file.svn_group.rev),
            &url,
            path.as_ref().map(PathBuf::as_path),
        )?
    }

    GitIgnore {
        exclude_all,
        exclude_paths,
        include_paths,
    }
    .to_file(Path::new("./"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svnadmin::SvnAdmin;
    use std::env;
    use std::fmt::{self, Display, Formatter};
    use std::io;
    use std::path;
    use tempfile::{tempdir, Builder};
    use walkdir::{DirEntry, WalkDir};

    #[derive(Debug)]
    pub enum TestError {
        LibError(Error),
        WalkDirError(walkdir::Error),
        StripPrefixError(path::StripPrefixError),
    }

    use self::TestError::*;

    impl From<Error> for TestError {
        fn from(e: Error) -> TestError {
            LibError(e)
        }
    }

    impl From<io::Error> for TestError {
        fn from(e: io::Error) -> TestError {
            LibError(Error::from(e))
        }
    }

    impl From<url::ParseError> for TestError {
        fn from(e: url::ParseError) -> TestError {
            LibError(Error::from(e))
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
        SvnAdmin::cmd()?.create(repo_dir.path())?;
        let repo_url = Url::parse(&format!("file://{}", repo_dir.path().display()))?;

        let svn = Svn::cmd()?;

        let entries1 = {
            let checkout_dir = tempdir()?;
            svn.checkout(false, None, &repo_url, Some(checkout_dir.path()))?;

            let file = Builder::new()
                .rand_bytes(10)
                .tempfile_in(checkout_dir.path())?;

            let pwd = env::current_dir()?;
            env::set_current_dir(checkout_dir.path())?;

            svn.add(false, &vec![file.path()])?;
            svn.commit(false, "new file", &[])?;

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
