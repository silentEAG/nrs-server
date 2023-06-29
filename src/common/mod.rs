// 定义数据结构体相关内容

use poem_openapi::{payload::Json, ApiResponse, Object};

pub mod data;
pub mod object;

#[derive(Object)]
pub struct ErrorMessage {
    pub message: String,
}

impl ErrorMessage {
    pub fn new<T: ToString>(msg: T) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

/// Api 异常处理
#[derive(ApiResponse)]
pub enum ApiError {
    /// 用户 auth 校验失败
    #[oai(status = 751)]
    UserAuthFailed,

    /// Admin auth 校验失败
    #[oai(status = 752)]
    AdminAuthFailed,

    /// 用户名不存在
    #[oai(status = 850)]
    UserNotExists,

    /// 用户名已经存在
    #[oai(status = 851)]
    UserAlreadyExists,

    /// 用户账号密码错误
    #[oai(status = 852)]
    UserPasswordError,

    /// 用户信息更新失败
    #[oai(status = 853)]
    UserUpdateFailed(Json<ErrorMessage>),

    /// 数据库错误
    #[oai(status = 854)]
    DBError(Json<ErrorMessage>),

    /// 签名失败
    #[oai(status = 855)]
    SignError(Json<ErrorMessage>),

    /// 标签更新失败
    #[oai(status = 856)]
    TagUpdateError(Json<Vec<String>>),

    /// 标签查询失败
    #[oai(status = 857)]
    TagQueryError(Json<ErrorMessage>),

    /// RPC错误
    #[oai(status = 858)]
    RPCError(Json<ErrorMessage>),

    /// 没有找到推荐的用户
    #[oai(status = 859)]
    NoRecommendUserFound,

    /// 其他错误
    #[oai(status = 860)]
    Error(Json<ErrorMessage>),
}

/// 无数据返回
#[derive(Object)]
pub struct NoData {}

pub type ApiResult<T> = Result<Json<T>, ApiError>;
