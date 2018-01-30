use std::iter::FromIterator;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::process::Command;
use std::path::PathBuf;
use semver::Version;
use failure::Error;
use slog::{self, Logger, Record, Serializer, KV};
use errors::{CommandFailed, FailedInvocation};

pub fn check_if_outdated(krate: &Crate, logger: Logger) -> Result<UpgradeSet, Error> {
    let logger = logger.new(o!("crate" => krate.name.clone()));

    // note: we specify a "special" exit code here because cargo exits with the
    // usual 101 code when it can't find a backend.
    let output = match Command::new("cargo")
        .arg("outdated")
        .arg("--root-deps-only")
        .arg("--exit-code")
        .arg("42")
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
    } else if output.status.code() == Some(42) {
        let output = String::from_utf8(output.stdout)?;
        parse_cargo_outdated_output(&output, &logger)
    } else {
        error!(logger, "Call to cargo-outdated failed"; 
            "stdout" => String::from_utf8_lossy(&output.stdout).to_string(),
            "stderr" => String::from_utf8_lossy(&output.stderr).to_string(),);
        Err(CommandFailed::new("cargo-outdated", output).into())
    }
}

fn parse_cargo_outdated_output(src: &str, logger: &Logger) -> Result<UpgradeSet, Error> {
    let mut set = UpgradeSet::empty();

    for (i, line) in src.lines().enumerate().skip(2) {
        trace!(logger, "Parsing line"; "line" => line, "line-number" => i);

        if let Some(upgrade) = parse_line(line, &logger) {
            debug!(logger, "Found upgradeable dependency"; &upgrade);
            set.insert(upgrade.name.clone(), upgrade);
        }
    }

    Ok(set)
}

/// Try to parse a single line of output from `cargo outdated`. Errors are
/// logged and ignored.
fn parse_line(line: &str, logger: &Logger) -> Option<Upgrade> {
    let columns: Vec<_> = line.split_whitespace().collect();
    if columns.len() != 6 {
        return None;
    }

    let name = columns[0].to_string();
    let from = columns[1];
    let to = if columns[2].contains("---") {
        columns[3]
    } else {
        columns[2]
    };

    let from = match from.parse::<Version>() {
        Ok(f) => f,
        Err(e) => {
            warn!(logger, "Unable to parse dependency version"; 
                    "crate-name" => name,
                    "from" => from,
                    "error" => e.to_string());
            return None;
        }
    };
    let to = match to.parse::<Version>() {
        Ok(f) => f,
        Err(e) => {
            warn!(logger, "Unable to parse dependency version"; 
                    "crate-name" => name,
                    "to" => to,
                    "error" => e.to_string());
            return None;
        }
    };

    Some(Upgrade { name, from, to })
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpgradeSet(HashMap<String, Upgrade>);

impl UpgradeSet {
    pub fn empty() -> UpgradeSet {
        UpgradeSet::default()
    }
}

impl<S: Into<String>> FromIterator<(S, Upgrade)> for UpgradeSet {
    fn from_iter<I: IntoIterator<Item = (S, Upgrade)>>(it: I) -> Self {
        let inner = it.into_iter().map(|(s, up)| (s.into(), up)).collect();
        UpgradeSet(inner)
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

#[derive(Debug, Clone, PartialEq)]
pub struct Upgrade {
    pub name: String,
    pub from: Version,
    pub to: Version,
}

impl KV for Upgrade {
    fn serialize(&self, _record: &Record, serializer: &mut Serializer) -> slog::Result {
        serializer.emit_str("name".into(), &self.name)?;
        serializer.emit_arguments("from".into(), &format_args!("{}", self.from))?;
        serializer.emit_arguments("to".into(), &format_args!("{}", self.to))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use slog::Discard;

    fn noop_logger() -> Logger {
        Logger::root(Discard, o!())
    }

    #[test]
    fn parse_a_single_line() {
        let line = "bitflags  0.1.1    ---     1.0.1   Normal  ---";
        let should_be = Upgrade {
            name: String::from("bitflags"),
            from: Version::parse("0.1.1").unwrap(),
            to: Version::parse("1.0.1").unwrap(),
        };

        let got = parse_line(line, &noop_logger()).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_some_actual_output() {
        let src = "
Name      Project  Compat  Latest  Kind    Platform
----      -------  ------  ------  ----    --------
bitflags  0.1.1    ---     1.0.1   Normal  ---
libc      0.1.12   ---     0.2.36  Normal  ---
        ";
        let should_be = vec![
            (
                "bitflags",
                Upgrade {
                    name: String::from("bitflags"),
                    from: Version::parse("0.1.1").unwrap(),
                    to: Version::parse("1.0.1").unwrap(),
                },
            ),
            (
                "libc",
                Upgrade {
                    name: String::from("libc"),
                    from: Version::parse("0.1.12").unwrap(),
                    to: Version::parse("0.2.36").unwrap(),
                },
            ),
        ];
        let should_be = UpgradeSet::from_iter(should_be);

        let got = parse_cargo_outdated_output(src, &noop_logger()).unwrap();
        assert_eq!(got, should_be);
    }
}
