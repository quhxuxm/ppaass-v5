use crate::error::Error;
use ppaass_common::log::LogConfig;
use serde::Deserialize;
use std::sync::LazyLock;

pub static PROXY_CONFIG: LazyLock<ProxyConfig> =
    LazyLock::new(|| init().expect("failed to init proxy config"));

/// The proxy configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ProxyConfig {
    port: Option<u16>,
    worker_threads: Option<usize>,
    log_config: LogConfig,
}

impl ProxyConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(80)
    }

    pub fn worker_threads(&self) -> usize {
        self.worker_threads.unwrap_or(256)
    }

    pub fn log_config(&self) -> &LogConfig {
        &self.log_config
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
