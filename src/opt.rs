use error::Error;
use structopt::StructOpt;

/// Command-line options
#[derive(StructOpt, Debug)]
pub struct Opt {
    /// Silence output
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Log level (error, warn, info, debug, trace)
    #[structopt(long = "log", default_value = "info", parse(try_from_str))]
    pub log_level: log::Level,
}

impl Opt {
    pub fn init() -> Result<Opt, Error> {
        let opt = Opt::from_args();

        stderrlog::new()
            .quiet(opt.quiet)
            .verbosity(opt.log_level as usize - 1)
            .init() // This returns an error if called twice.
            .unwrap(); // We should never do that, so panic if we do.

        log::debug!("options: {:?}", opt);
        Ok(opt)
    }
}
