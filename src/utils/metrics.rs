use {
    std::{thread, time::Duration, fs, path::Path, collections::HashMap},
    prometheus::{Registry, TextEncoder},
    serde::Deserialize,
};

#[derive(Deserialize)]
pub struct MetricsConfig {
    enabled: Option<bool>,
    endpoint: String,
    username: String,
    password: String,
}

impl MetricsConfig {
    pub fn load_from_config() -> Self {
        let config_path  = "config/metrics.toml";
        if !Path::new(config_path).exists() {
            return Self::default();
        }

        toml::from_str(&fs::read_to_string(config_path).unwrap()).unwrap()
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            endpoint: "".to_owned(),
            username: "".to_owned(),
            password: "".to_owned(),
        }
    }
}

pub fn init_metrics(run_id: Option<String>) -> Registry {
    let registry = metrics_registry(run_id);

    start_metrics_push_thread(registry.clone(), MetricsConfig::load_from_config());

    registry
}

pub fn metrics_registry(run_id: Option<String>) -> Registry {
    let mut labels = HashMap::new();
    labels.insert("node".to_owned(), hostname::get().unwrap().to_string_lossy().to_string());
    if let Some(run_id) = run_id {
        labels.insert("run_id".to_owned(), run_id);
    }

    Registry::new_custom(Some("far_memory".to_owned()), Some(labels)).unwrap()
}

pub fn start_metrics_push_thread(registry: Registry, metrics_config: MetricsConfig) {
    if !metrics_config.enabled.unwrap_or(true) {
        return;
    }

    thread::spawn(move || {
        let encoder = TextEncoder::new();
        let client = reqwest::blocking::Client::new();

        loop {
            let metrics = registry.gather();
            let encoded = encoder.encode_to_string(&metrics).unwrap();

            client.post(&metrics_config.endpoint)
                .basic_auth(&metrics_config.username, Some(&metrics_config.password))
                .body(encoded)
                .send()
                .unwrap();

            thread::sleep(Duration::from_secs(10));
        }
    });
}
