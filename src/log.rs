use tracing::info;
use tracing_appender::{rolling, non_blocking};
use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use crate::config::CONFIG;


#[inline(always)]
pub fn start() -> anyhow::Result<()> {
    // Enable tracing logger
    // 1. stdout layer
    let formatting_layer = fmt::layer();
    // 2. server file layer
    let file_appender = rolling::daily("logs", CONFIG.server.log_file.clone());
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let server_file_layer = fmt::layer()
        .with_ansi(false)
        .with_line_number(true)
        .with_file(true)
        .with_writer(non_blocking_appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| CONFIG.common.log_level.clone()),
        ))
        .with(server_file_layer)
        .with(formatting_layer)
        .init();
    info!("Tracing logger initialized");
    Ok(())
}