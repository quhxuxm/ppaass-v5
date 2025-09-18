use crate::error::Error;
use serde::Deserialize;
use std::borrow::ToOwned;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::LazyLock;
use tracing::debug;
use tracing_subscriber::filter::LevelFilter;



pub static PROXY_CONFIG: LazyLock<ProxyConfig> =
    LazyLock::new(|| init().expect("failed to init proxy config"));

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "log_type", content = "value")]
pub enum LogType {
    Fs {
        log_dir: Option<PathBuf>,
        log_name_prefix: Option<String>,
    },
    Stdout,
}
/// The proxy configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ProxyConfig {
    port: Option<u16>,
    worker_threads: Option<usize>,
    log_type: LogType,
    log_max_level: Option<String>,
}

impl ProxyConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(80)
    }

    pub fn worker_threads(&self) -> usize {
        self.worker_threads.unwrap_or(256)
    }

    pub fn log_max_level(&self) -> LevelFilter {
        self.log_max_level
            .as_ref()
            .map(|v| LevelFilter::from_str(v).unwrap_or(LevelFilter::INFO))
            .unwrap_or(LevelFilter::INFO)
    }
    pub fn log_type(&self) -> &LogType {
        &self.log_type
    }
}

/// Initialize the proxy configuration
pub fn init() -> Result<ProxyConfig, Error> {
    let config = config::Config::builder()
        .add_source(config::File::with_name("proxy"))
        .add_source(config::Environment::with_prefix("PPAASS_PROXY"))
        .build()?;
    let proxy_config = config.try_deserialize::<ProxyConfig>()?;
    Ok(proxy_config)
}
