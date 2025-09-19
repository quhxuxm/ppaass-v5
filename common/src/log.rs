use crate::error::Error;
use serde::Deserialize;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::time::ChronoUtc;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

static DEFAULT_LOG_DIR_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from_str("./log").expect("failed to define default log directory"));

static DEFAULT_LOG_FILE_NAME_PREFIX: LazyLock<String> =
    LazyLock::new(|| "ppaass-proxy.log".to_owned());

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LogConfig {
    output: LogOutput,
    max_level: Option<String>,
}

impl LogConfig {
    pub fn output(&self) -> &LogOutput {
        &self.output
    }

    pub fn max_level(&self) -> LevelFilter {
        self.max_level
            .as_ref()
            .map(|v| LevelFilter::from_str(v).unwrap_or(LevelFilter::INFO))
            .unwrap_or(LevelFilter::INFO)
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(tag = "type")]
pub enum LogOutput {
    File {
        output_dir: Option<PathBuf>,
        output_filename_prefix: Option<String>,
    },
    #[default]
    Stdout,
}

pub fn init(config: &LogConfig) -> Result<Option<WorkerGuard>, Error> {
    let (trace_log_writer, trace_appender_guard) = match config.output() {
        LogOutput::File {
            output_dir: log_dir,
            output_filename_prefix: log_name_prefix,
        } => {
            let log_dir = log_dir.as_ref().unwrap_or(&DEFAULT_LOG_DIR_PATH);
            let log_name_prefix = log_name_prefix
                .as_ref()
                .unwrap_or(&DEFAULT_LOG_FILE_NAME_PREFIX);
            let (trace_log_writer, trace_appender_guard) = tracing_appender::non_blocking(
                tracing_appender::rolling::daily(log_dir, log_name_prefix),
            );
            (
                BoxMakeWriter::new(trace_log_writer),
                Some(trace_appender_guard),
            )
        }
        LogOutput::Stdout => {
            let std_writer = Arc::new(std::io::stdout());
            (BoxMakeWriter::new(std_writer), None)
        }
    };
    let registry = tracing_subscriber::registry();
    registry
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(config.max_level().to_string().as_str())),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(trace_log_writer)
                .with_line_number(true)
                .with_level(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_timer(ChronoUtc::rfc_3339())
                .with_ansi(false),
        )
        .init();
    Ok(trace_appender_guard)
}
