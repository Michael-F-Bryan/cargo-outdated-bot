use std::path::PathBuf;
use slog::{self, Record, Serializer, Value, KV};
use config::{Config, Environment};
use failure::Error;
use serde_json;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub index_dir: PathBuf,
    pub port: usize,
}

impl Settings {
    /// Load the `Settings` from the environment.
    ///
    /// This will use the *SHOUTY_SNAKE_CASE* version of each element's name,
    /// falling back to the defaults if not present.
    ///
    /// # Examples
    ///
    /// If you wanted to set the `port` to `1337` and `index_dir` to `/`, you
    /// would use something like this:
    ///
    /// ```text
    /// $ PORT=1337 INDEX_DIR=/ cargo run
    /// ```
    pub fn from_env() -> Result<Settings, Error> {
        let mut s = Config::new();

        let defaults = Settings::default();
        s.cache = serde_json::from_value(serde_json::to_value(defaults)?)?;

        let mut env = Environment::new();
        env.separator("__".to_string());
        s.merge(env)?;

        s.try_into().map_err(Error::from)
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            index_dir: PathBuf::from("/crates"),
            port: 8000,
        }
    }
}

impl KV for Settings {
    fn serialize(&self, record: &Record, ser: &mut Serializer) -> slog::Result {
        self.index_dir
            .display()
            .serialize(record, "index-dir".into(), ser)?;
        ser.emit_usize("port".into(), self.port)?;

        Ok(())
    }
}
