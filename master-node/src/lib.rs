//! Master node for the `cargo-outdated-bot`.
//!
//! This master node is in charge of:
//!
//! - Keeping track of which crates are on crates.io and what their latest
//!   versions are
//! - Handing out jobs for nodes to execute
//! - Recording the result of each job

extern crate configure;
#[macro_use]
extern crate configure_derive;
extern crate crates_index;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;

mod config;
mod master;

pub use config::Config;
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
