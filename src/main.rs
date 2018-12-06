extern crate manifest;

use std::process;

fn main() {
    manifest::run().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}
