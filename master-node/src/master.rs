use failure::Error;
use crates_index::Index;
use slog::Logger;

use config::Config;
use SyncResult;

#[derive(Debug)]
pub struct Master {
    index: Index,
    logger: Logger,
}

impl Master {
    pub fn new(cfg: &Config, root_logger: &Logger) -> Result<Master, Error> {
        let index = Index::new(cfg.index_dir.clone());
        let logger = root_logger.new(o!(
            "version" => env!("CARGO_PKG_VERSION"), 
            "name" => env!("CARGO_PKG_NAME")
           ));

        let m = Master { index, logger };
        m.update()?;

        Ok(m)
    }

    pub fn update(&self) -> Result<(), Error> {
        info!(self.logger, "Updating the master");

        debug!(self.logger, "Retrieving index");
        self.index.retrieve_or_update().sync()?;
        debug!(self.logger, "Index up-to-date");

        Ok(())
    }
}
