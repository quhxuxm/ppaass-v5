use std::path::PathBuf;
use std::str::FromStr;
use std::sync::LazyLock;
use crate::config::{LogType, PROXY_CONFIG};
use crate::error::Error;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::ChronoUtc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::MakeWriter;

static DEFAULT_LOG_DIR_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from_str("./log").expect("failed to define default log directory"));

static DEFAULT_LOG_FILE_NAME_PREFIX: LazyLock<String> =
    LazyLock::new(|| "ppaass-proxy.log".to_owned());
pub fn init() -> Result<WorkerGuard, Error> {
    let log_type = PROXY_CONFIG.log_type();

    let (trace_log_writer, trace_appender_guard) = match log_type {
        LogType::Fs { log_dir, log_name_prefix } => {
            let log_dir = log_dir.as_ref().unwrap_or(&DEFAULT_LOG_DIR_PATH);
            let log_name_prefix=log_name_prefix.as_ref().unwrap_or(&DEFAULT_LOG_FILE_NAME_PREFIX);
         let (trace_log_writer, trace_appender_guard)=   tracing_appender::non_blocking(
                tracing_appender::rolling::daily(log_dir, log_name_prefix),
            );
            (Box::new(trace_log_writer), Some(trace_appender_guard))
        },
        LogType::Stdout => (Box::new(MakeWriter::make_writer()), None)
    };
    let registry = tracing_subscriber::registry();
    registry
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                EnvFilter::new(PROXY_CONFIG.log_max_level().to_string().as_str())
            }),
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
