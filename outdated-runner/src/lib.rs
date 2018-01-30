extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate structopt;

pub mod errors;

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::process::Command;
use std::path::PathBuf;
use failure::Error;
use slog::{Logger, Record, Serializer, KV};
use errors::FailedInvocation;

pub fn check_if_outdated(krate: &Crate, logger: Logger) -> Result<UpgradeSet, Error> {
    let logger = logger.new(o!("crate" => krate.name.clone()));

    let output = match Command::new("cargo")
        .arg("outdated")
        .arg("--root-deps-only")
        .arg("--exit-code")
        .arg("101")
        .current_dir(&krate.dir)
        .output()
    {
        Ok(out) => out,
        Err(e) => {
            error!(logger, "Invoking cargo-outdated failed"; "error" => e.to_string());
            return Err(FailedInvocation::new("cargo-outdate", e).into());
        }
    };

    debug!(logger, "Command finished"; "return-code" => output.status.code().unwrap_or(0));

    if output.status.success() {
        Ok(UpgradeSet::empty())
    } else if output.status.code() == Some(101) {
        let output = String::from_utf8(output.stdout)?;
        parse_cargo_outdated_output(&output, logger)
    } else {
        error!(logger, "Call to cargo-outdated failed"; 
            "stdout" => String::from_utf8_lossy(&output.stdout).to_string(),
            "stderr" => String::from_utf8_lossy(&output.stderr).to_string(),
            );
        Err(failure::err_msg("Unknown Return Code"))
    }
}

fn parse_cargo_outdated_output(src: &str, logger: Logger) -> Result<UpgradeSet, Error> {
    let mut set = UpgradeSet::empty();

    for (i, line) in src.lines().enumerate().skip(2) {
        trace!(logger, "Parsing line"; "line" => line, "line-number" => i);

        let columns: Vec<_> = line.split_whitespace().collect();
        if columns.len() != 6 {
            continue;
        }

        let name = columns[0];
        let from = columns[1];
        let to = if columns[2].contains("---") {
            columns[3]
        } else {
            columns[2]
        };

        let ug = Upgrade {
            name: name.to_string(),
            from: from.to_string(),
            to: to.to_string(),
        };

        debug!(logger, "Found upgradeable dependency"; &ug);
        set.insert(name.to_string(), ug);
    }

    Ok(set)
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpgradeSet(HashMap<String, Upgrade>);

impl UpgradeSet {
    pub fn empty() -> UpgradeSet {
        UpgradeSet::default()
    }
}

impl Deref for UpgradeSet {
    type Target = HashMap<String, Upgrade>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UpgradeSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Upgrade {
    pub name: String,
    pub from: String,
    pub to: String,
}

impl KV for Upgrade {
    fn serialize(&self, _record: &Record, serializer: &mut Serializer) -> slog::Result {
        serializer.emit_str("name".into(), &self.name)?;
        serializer.emit_str("from".into(), &self.from)?;
        serializer.emit_str("to".into(), &self.to)?;
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
