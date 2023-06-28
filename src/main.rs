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

/// 日志模块
mod log;

/// 工具模块
mod util;

/// RPC 模块
mod rpc;

/// Server 启动主要模块
mod server;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() {
    // 初始化日志
    log::start().unwrap();

    // Server 启动
    if let Err(e) = server::run().await {
        tracing::error!("server error: {}", e);
    }
}
