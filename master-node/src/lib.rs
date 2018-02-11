//! Master node for the `cargo-outdated-bot`.
//!
//! This master node is in charge of:
//!
//! - Keeping track of which crates are on crates.io and what their latest
//!   versions are
//! - Handing out jobs for nodes to execute
//! - Recording the result of each job

extern crate config;
extern crate crates_index;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate slog;

mod settings;
mod master;

pub use settings::Settings;
pub use master::Master;

use std::error::Error as StdError;
use failure::SyncFailure;

pub(crate) trait SyncResult<T, E>: Sized {
    fn sync(self) -> Result<T, SyncFailure<E>>;
}

impl<T, E: Send + StdError + 'static> SyncResult<T, E> for Result<T, E> {
    fn sync(self) -> Result<T, SyncFailure<E>> {
        self.map_err(SyncFailure::new)
    }
}
