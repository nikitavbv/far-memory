use {
    std::{path::Path, fs::read_to_string, env::var},
    serde::Deserialize,
};

#[derive(Default, Deserialize)]
pub struct Config {
    endpoint: Option<String>,
    access_token: Option<String>,

    memory_storage_enabled: Option<bool>,
    block_storage_client_enabled: Option<bool>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = var("FAR_MEMORY_CONFIG").unwrap_or("config.toml".to_owned());
        let config_path = Path::new(&config_path);
        if !config_path.exists() {
            return Self::default();
        }

        let config = read_to_string(config_path).unwrap();
        toml::from_str(&config).unwrap()
    }

    pub fn endpoint(&self) -> String {
        self.endpoint.as_ref().unwrap().clone()
    }

    pub fn access_token(&self) -> String {
        self.access_token.as_ref().unwrap().clone()
    }

    pub fn memory_storage_enabled(&self) -> bool {
        self.memory_storage_enabled.unwrap_or(false)
    }

    pub fn block_storage_client_enabled(&self) -> bool {
        self.block_storage_client_enabled.unwrap_or(false)
    }
}