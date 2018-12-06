use error::Error;
use std::ffi::OsString;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use url::Url;
use which::which;

#[derive(Debug)]
pub struct Svn {
    path: PathBuf,
    version: String,
}

impl<'a> duct::ToExecutable for &'a Svn {
    fn to_executable(self) -> OsString {
        self.path().to_executable()
    }
}

impl Svn {
    pub fn new() -> Result<Svn, Error> {
        let path = which(String::from("svn"))?;
        let cmd = duct::cmd!(&path, "--version", "--quiet");
        let svn = Svn {
            path,
            version: cmd.read()?,
        };
        log::debug!("svn: {:?}", &svn);
        Ok(svn)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    // http://svnbook.red-bean.com/en/1.7/svn.ref.svn.c.add.html
    pub fn add(&self, quiet: bool, paths: Vec<&Path>) -> Result<(), Error> {
        let mut args: Vec<OsString> = Vec::with_capacity(2 + paths.len());
        args.push(OsString::from("add"));
        if quiet {
            args.push(OsString::from("--quiet"));
        }
        let mut paths: Vec<OsString> = paths.iter().map(OsString::from).collect();
        args.append(&mut paths);
        let cmd = duct::cmd(self, args);
        log::debug!("cmd: {:?}", cmd);
        println!("{}", cmd.read()?);
        Ok(())
    }

    // http://svnbook.red-bean.com/en/1.7/svn.ref.svn.c.checkout.html
    pub fn checkout(
        &self,
        quiet: bool,
        rev: Option<usize>, // https://svn.haxx.se/users/archive-2005-03/0394.shtml
        url: &Url,
        path: Option<&Path>,
    ) -> Result<(), Error> {
        let mut args: Vec<OsString> = Vec::with_capacity(6);
        args.push(OsString::from("checkout"));
        if quiet {
            args.push(OsString::from("--quiet"));
        }
        if let Some(rev) = rev {
            args.push(OsString::from("--revision"));
            args.push(OsString::from(rev.to_string()));
        }
        args.push(OsString::from(url.as_str()));
        if let Some(path) = path {
            args.push(OsString::from(path));
        }
        let cmd = duct::cmd(self, args);
        log::debug!("cmd: {:?}", cmd);

        // The `svn checkout` command can be slow and print output incrementally. So we open a pipe
        // to the output, set the pipe input up as the command's `stdout`, and then loop through
        // buffered lines of the pipe output as it comes in.
        // https://github.com/oconnor663/duct.rs/issues/69

        let (pipe_out, pipe_in) = os_pipe::pipe()?;
        let child = cmd.stdout_handle(pipe_in).start()?;

        // We need to drop the duct::Expression at this point -- because it holds the
        // os_pipe::PipeWriter -- to avoid blocking the os_pipe::PipeReader loop below.
        drop(cmd);

        if !quiet {
            for line in BufReader::new(pipe_out).lines() {
                println!("{}", line?);
            }
        }

        child.wait()?;

        Ok(())
    }

    // http://svnbook.red-bean.com/en/1.7/svn.ref.svn.c.commit.html
    pub fn commit(&self, quiet: bool, msg: &str, paths: Vec<&Path>) -> Result<(), Error> {
        let mut args: Vec<OsString> = Vec::with_capacity(4 + paths.len());
        args.push(OsString::from("commit"));
        if quiet {
            args.push(OsString::from("--quiet"));
        }
        args.push(OsString::from("-m"));
        args.push(OsString::from(msg));
        let mut paths: Vec<OsString> = paths.iter().map(OsString::from).collect();
        args.append(&mut paths);
        let cmd = duct::cmd(self, args);
        log::debug!("cmd: {:?}", cmd);
        println!("{}", cmd.read()?);
        Ok(())
    }
}
