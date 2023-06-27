use poem::{web::Data, Endpoint};
use poem_openapi::{
    param::{Header, Query},
    payload::Json,
    OpenApi, Tags,
};

use crate::{
    common::{
        data::DbPool,
        object::{self, news, user},
        ApiResult, NoData,
    },
    config::{AppAuthorization, ServerKey, API_KEY},
    controller,
};

pub struct CommonApi;
pub struct AdminApi;
pub struct UserApi;
pub struct NewsApi;

/// OpenApi 标签
#[derive(Tags)]
enum ApiTags {
    /// 用户路由
    User,
    /// Admin 路由
    Admin,
    /// 新闻路由
    News,
    /// 其他路由
    Common,
}

/// 其他路由
#[OpenApi]
impl CommonApi {
    /// 心跳检测路由
    #[oai(path = "/ping", method = "get", tag = "ApiTags::Common")]
    async fn ping(&self) -> Json<serde_json::Value> {
        Json(serde_json::json!({
            "message": "pong",
        }))
    }
}

/// 用户路由
#[OpenApi(prefix_path = "/user")]
impl UserApi {
    /// 用户登录路由
    #[oai(path = "/login", method = "post", tag = "ApiTags::User")]
    async fn login(
        &self,
        Json(user): Json<user::LoginRequest>,
        Data(pool): Data<&DbPool>,
        Data(server_key): Data<&ServerKey>,
    ) -> ApiResult<user::LoginSuccess> {
        controller::user::login(pool, server_key, user).await
    }

    /// 用户注册路由
    #[oai(path = "/register", method = "post", tag = "ApiTags::User")]
    async fn register(
        &self,
        Json(user): Json<user::RegisterRequest>,
        Data(pool): Data<&DbPool>,
    ) -> ApiResult<NoData> {
        controller::user::register(pool, user).await
    }

    /// 个人认证测试路由，需要 user 认证
    #[oai(path = "/auth", method = "get", tag = "ApiTags::User")]
    async fn auth(&self, _auth: AppAuthorization) -> ApiResult<NoData> {
        Ok(Json(NoData {}))
    }

    /// 更新个人信息路由，需要 user 认证
    #[oai(path = "/update", method = "post", tag = "ApiTags::User")]
    async fn update(
        &self,
        Json(update_info): Json<user::UpdateRequest>,
        Data(pool): Data<&DbPool>,
        auth: AppAuthorization,
    ) -> ApiResult<NoData> {
        controller::user::update(pool, auth.0.id, update_info).await
    }

    /// 获取个人信息路由，需要 user 认证
    #[oai(path = "/info", method = "get", tag = "ApiTags::User")]
    async fn info(
        &self,
        Data(pool): Data<&DbPool>,
        auth: AppAuthorization,
    ) -> ApiResult<user::InfoResponse> {
        controller::user::get_info(pool, auth.0.id).await
    }

    /// 获取个人历史浏览记录路由，需要 user 认证
    #[oai(path = "/history", method = "get", tag = "ApiTags::User")]
    async fn history(
        &self,
        Data(pool): Data<&DbPool>,
        auth: AppAuthorization,
    ) -> ApiResult<user::HistoryResponse> {
        controller::user::get_history(pool, auth.0.id).await
    }
}

/// Admin 路由
#[OpenApi(prefix_path = "/admin")]
impl AdminApi {
    /// 指定用户信息路由，需要 admin 认证
    #[oai(path = "/userinfo", method = "get", tag = "ApiTags::Admin")]
    async fn user_info(
        &self,
        Data(pool): Data<&DbPool>,
        Query(user_id): Query<i32>,
        #[oai(name = "ADMIN-TOKEN")] token: Header<String>,
    ) -> ApiResult<user::InfoResponse> {
        if token.0 != API_KEY {
            return Err(crate::common::ApiError::AdminAuthFailed);
        }
        controller::admin::get_user_by_id(pool, user_id).await
    }

    /// 创建新闻路由，需要 admin 认证
    #[oai(path = "/createnews", method = "post", tag = "ApiTags::Admin")]
    async fn create_news(
        &self,
        news: Json<object::news::CreateNewsRequest>,
        #[oai(name = "ADMIN-TOKEN")] token: Header<String>,
    ) -> ApiResult<Vec<user::InfoResponse>> {
        if token.0 != API_KEY {
            return Err(crate::common::ApiError::AdminAuthFailed);
        }
        // TODO
        unimplemented!()
    }
}

/// 新闻路由
#[OpenApi(prefix_path = "/news")]
impl NewsApi {
    /// 用户获取推荐新闻路由，需要用户认证
    #[oai(path = "/recommend", method = "get", tag = "ApiTags::News")]
    async fn recommend(&self, auth: AppAuthorization) -> ApiResult<Vec<news::AbstractResponse>> {
        unimplemented!()
    }

    /// 获取指定新闻路由，需要用户认证
    #[oai(path = "/get", method = "get", tag = "ApiTags::News")]
    async fn get(
        &self,
        news_id: Query<String>,
        auth: AppAuthorization,
    ) -> ApiResult<news::DetailResponse> {
        unimplemented!()
    }

    /// like 指定新闻路由，需要用户认证
    #[oai(path = "/like", method = "get", tag = "ApiTags::News")]
    async fn like(&self, news_id: Query<String>, auth: AppAuthorization) -> ApiResult<NoData> {
        unimplemented!()
    }
}
