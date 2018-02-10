use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Configure)]
#[serde(default)]
pub struct Config {
    pub index_dir: PathBuf,
    pub port: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            index_dir: PathBuf::from("/crates"),
            port: 8000,
        }
    }
}
