use hmac::digest::KeyInit;
use hmac::Hmac;
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use sha2::Sha256;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use crate::{
    api::{AdminApi, CommonApi, NewsApi, UserApi},
    config::SERVER_KEY,
};

/// 日志模块
mod log;

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

/// 工具模块
mod util;

#[tokio::main]
async fn main() {
    // 初始化日志
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    info!("Starting news recommend server...");

    // 初始化数据库连接池
    let pool = PgPoolOptions::new()
        .min_connections(5)
        .max_connections(10)
        .connect("postgres://news_recommender:nekopara@127.0.0.1:5432/news_recommend")
        .await
        .unwrap();

    // 初始化 server key
    let server_key = Hmac::<Sha256>::new_from_slice(SERVER_KEY).expect("valid server key");

    // 初始化 OpenApi 服务
    let api_service: OpenApiService<(CommonApi, AdminApi, UserApi, NewsApi), ()> =
        OpenApiService::new(
            (CommonApi, AdminApi, UserApi, NewsApi),
            "News Recommend Server",
            "1.0",
        )
        .server("http://localhost:3000/api");

    // 初始化 swagger-ui 服务
    let ui = api_service.swagger_ui();

    let spec = api_service.spec();

    // 初始化路由
    let router = Route::new()
        .nest("/api", api_service)
        .nest("/docs", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .data(pool)
        .data(server_key);

    // 启动服务器
    let _ = Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(router)
        .await;
}
