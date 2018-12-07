use error::Error;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use which::which;

#[derive(Debug)]
pub struct SvnAdmin {
    path: PathBuf,
    version: String,
}

impl<'a> duct::ToExecutable for &'a SvnAdmin {
    fn to_executable(self) -> OsString {
        self.path().to_executable()
    }
}

impl SvnAdmin {
    pub fn init() -> Result<SvnAdmin, Error> {
        let path = which(String::from("svnadmin"))?;
        let cmd = duct::cmd!(&path, "--version", "--quiet");
        let svnadmin = SvnAdmin {
            path,
            version: cmd.read()?,
        };
        log::debug!("svnadmin: {:?}", &svnadmin);
        Ok(svnadmin)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    // http://svnbook.red-bean.com/en/1.7/svn.ref.svnadmin.c.create.html
    pub fn create(&self, path: &Path) -> Result<(), Error> {
        let args: Vec<OsString> = vec![OsString::from("create"), OsString::from(path)];
        let cmd = duct::cmd(self, args);
        log::debug!("cmd: {:?}", cmd);
        println!("{}", cmd.read()?);
        Ok(())
    }
}
