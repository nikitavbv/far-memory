use {
    std::{path::Path, fs::read_to_string},
    serde::Deserialize,
};

#[derive(Default, Deserialize)]
pub struct Config {
    access_token: Option<String>,

    memory_storage_enabled: Option<bool>,
    block_storage_client_enabled: Option<bool>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = Path::new("config.toml");
        if !config_path.exists() {
            return Self::default();
        }

        let config = read_to_string(config_path).unwrap();
        toml::from_str(&config).unwrap()
    }

    pub fn memory_storage_enabled(&self) -> bool {
        self.memory_storage_enabled.unwrap_or(false)
    }

    pub fn block_storage_client_enabled(&self) -> bool {
        self.block_storage_client_enabled.unwrap_or(false)
    }
}