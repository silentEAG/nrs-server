use hmac::digest::KeyInit;
use hmac::Hmac;
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use sha2::Sha256;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use crate::{
    api::{AdminApi, CommonApi, NewsApi, UserApi}, backend, config::CONFIG,
};

pub async fn run() -> anyhow::Result<()> {

    info!("Starting news recommend system server (NRS-Server)");

    info!("Starting to connect to database");

    // 初始化数据库连接池
    let database_url = format!("postgres://{}:{}@{}:{}/{}",
        CONFIG.database.user_name,
        CONFIG.database.password,
        CONFIG.database.host,
        CONFIG.database.port,
        CONFIG.database.db
    );
    let pool = PgPoolOptions::new()
        .min_connections(5)
        .max_connections(15)
        .connect(&database_url)
        .await?;

    info!("Starting to set backend tasks...");
    // 启动后台任务
    backend::start(pool.clone()).await?;

    info!("Starting to initialize server");
    // 初始化 server key
    let server_key = Hmac::<Sha256>::new_from_slice(CONFIG.server.server_key.as_bytes())?;

    // 初始化 OpenApi 服务
    let api_url = format!("http://localhost:{}/api", CONFIG.server.api_port);
    let api_service: OpenApiService<(CommonApi, AdminApi, UserApi, NewsApi), ()> =
        OpenApiService::new(
            (CommonApi, AdminApi, UserApi, NewsApi),
            "News Recommend Server",
            "1.0",
        )
        .server(api_url);

    // 初始化 swagger-ui 服务
    let ui = api_service.swagger_ui();
    let spec = api_service.spec();

    // 初始化 API 路由
    let router = Route::new()
        .nest("/api", api_service);

    // 仅在开发环境下开放 api 路由
    #[cfg(debug_assertions)]
    let router = router
        .nest("/docs", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()));

    // 增加共享数据以及日志中间件
    let router = router
        .with(poem::middleware::Tracing)
        .data(pool)
        .data(server_key);


    // 启动服务器
    let server_url = format!("0.0.0.0:{}", CONFIG.server.api_port);
    Server::new(TcpListener::bind(server_url))
        .run(router)
        .await?;

    Ok(())
}