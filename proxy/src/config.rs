use crate::error::Error;
use serde::Deserialize;
use std::borrow::ToOwned;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::LazyLock;
use tracing::debug;
use tracing_subscriber::filter::LevelFilter;

static DEFAULT_LOG_DIR_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from_str("./log").expect("failed to define default log directory"));

static DEFAULT_LOG_FILE_NAME_PREFIX: LazyLock<String> =
    LazyLock::new(|| "ppaass-proxy.log".to_owned());

pub static PROXY_CONFIG: LazyLock<ProxyConfig> =
    LazyLock::new(|| init().expect("failed to init proxy config"));

/// The proxy configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ProxyConfig {
    port: Option<u16>,
    worker_threads: Option<usize>,
    log_dir: Option<PathBuf>,
    log_name_prefix: Option<String>,
    log_max_level: Option<String>,
}

impl ProxyConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(80)
    }

    pub fn worker_threads(&self) -> usize {
        self.worker_threads.unwrap_or(256)
    }

    pub fn log_dir(&self) -> &Path {
        &self.log_dir.as_ref().unwrap_or(&DEFAULT_LOG_DIR_PATH)
    }

    pub fn log_name_prefix(&self) -> &str {
        &self
            .log_name_prefix
            .as_ref()
            .unwrap_or(&DEFAULT_LOG_FILE_NAME_PREFIX)
    }

    pub fn log_max_level(&self) -> LevelFilter {
        self.log_max_level
            .as_ref()
            .map(|s| s.parse().unwrap_or(LevelFilter::INFO))
            .unwrap_or(LevelFilter::INFO)
    }
}

/// Initialize the proxy configuration
pub fn init() -> Result<ProxyConfig, Error> {
    let config = config::Config::builder()
        .add_source(config::File::with_name("proxy"))
        .add_source(config::Environment::with_prefix("PPAASS_PROXY"))
        .build()?;
    let proxy_config = config.try_deserialize::<ProxyConfig>()?;
    debug!("read proxy config success: {:#?}", proxy_config);
    Ok(proxy_config)
}
