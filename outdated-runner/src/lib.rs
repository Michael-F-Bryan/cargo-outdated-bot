extern crate failure;
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::process::Command;
use std::path::PathBuf;
use failure::{Error, ResultExt};
use slog::Logger;

pub fn check_if_outdated(krate: &Crate, logger: Logger) -> Result<(), Error> {
    let logger = logger.new(o!("crate" => krate.name.clone(), 
                      "dir" => krate.dir.display().to_string()));

    let output = match Command::new("cargo")
        .arg("outdated")
        .arg("--exit-code")
        .arg("101")
        .current_dir(&krate.dir)
        .output()
    {
        Ok(out) => out,
        Err(e) => {
            error!(logger, "Invoking cargo-outdated failed"; "error" => e.to_string());
            return Err(e)
                .context("Invoking cargo-outdated failed")
                .map_err(Error::from);
        }
    };

    debug!(logger, "Command finished"; 
        "return-code" => output.status.code().unwrap_or(0),
        "stdout" => String::from_utf8_lossy(&output.stdout).to_string(),
        "stderr" => String::from_utf8_lossy(&output.stderr).to_string(),
    );

    if output.status.code() == Some(101) {
        panic!("There were errors")
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Crate {
    name: String,
    dir: PathBuf,
}

impl Crate {
    pub fn from_dir<P: Into<PathBuf>>(dir: P) -> Crate {
        let dir = dir.into();
        let name = dir.file_stem().unwrap().to_string_lossy().to_string();

        Crate { name, dir }
    }
}
