use failure::Error;
use crates_index::Index;
use slog::Logger;

use settings::Settings;
use SyncResult;

/// The object tracking the master node's state, allowing you to work directly
/// with the underlying data.
#[derive(Debug)]
pub struct Master {
    index: Index,
    logger: Logger,
}

impl Master {
    pub fn new(cfg: &Settings, root_logger: &Logger) -> Master {
        let index = Index::new(cfg.index_dir.clone());
        let logger = root_logger.new(o!(
            "version" => env!("CARGO_PKG_VERSION"), 
            "name" => env!("CARGO_PKG_NAME")
           ));

        Master { index, logger }
    }

    pub fn update(&self) -> Result<(), Error> {
        info!(self.logger, "Updating the master");

        debug!(self.logger, "Retrieving index");
        self.index.retrieve_or_update().sync()?;
        debug!(self.logger, "Index up-to-date");

        Ok(())
    }
}
