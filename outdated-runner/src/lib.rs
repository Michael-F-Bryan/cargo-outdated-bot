extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate structopt;
extern crate semver;

pub mod errors;
mod check;

pub use check::{check_if_outdated, Crate, Upgrade, UpgradeSet};
