/// 配置模块
mod config;

/// 公共模块
mod common;

/// API 层
/// 所有的 API 路由都在这里定义接口，然后在 controller 层实现
/// - CommonApi: 公共路由
/// - AdminApi: Admin 路由
/// - UserApi: 用户路由
/// - NewsApi: 新闻路由
mod api;

/// Controller 层, 实现 API 层定义的接口
mod controller;

/// 后台任务模块
mod backend;

/// 工具模块
mod util;

/// RPC 模块
mod rpc;

/// Server 启动主要模块
mod server;

#[cfg(test)]
mod test;

use tracing::info;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

use crate::config::CONFIG;

#[tokio::main]
async fn main() {
    // 初始化日志
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

    // Server 启动
    if let Err(e) = server::run().await {
        tracing::error!("server error: {}", e);
    }
}
