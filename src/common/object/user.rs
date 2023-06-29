use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use crate::common::data::user::UserData;

use super::news;

/// 用户登录请求
#[derive(Object)]
pub struct LoginRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

/// 用户登录成功返回 token
#[derive(Object)]
pub struct LoginSuccess {
    /// 用户 token
    pub token: String,
}

/// 用户注册请求
#[derive(Object)]
pub struct RegisterRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 年龄
    pub age: i32,
    /// 性别
    pub sex: Option<String>,
}

/// 用户信息响应
#[derive(Object)]
pub struct InfoResponse {
    /// 用户名
    pub username: String,
    /// 兴趣 tag
    pub interests: Vec<String>,
    /// 年龄
    pub age: i32,
    /// 性别
    pub sex: String,
    /// 创建时间
    pub create_time: chrono::NaiveDateTime,
}

/// 用户信息更新请求
#[derive(Object)]
pub struct UpdateRequest {
    /// 兴趣 tag
    pub interests: Option<Vec<String>>,
    /// 密码
    pub password: Option<String>,
}

/// 用户历史记录响应
#[derive(Object)]
pub struct HistoryResponse {
    /// 历史记录
    pub news: Vec<news::AbstractResponse>,
}

#[derive(Serialize, Deserialize, Object, PartialEq, Eq, Hash)]
pub struct UserSign {
    pub id: i32,
    pub username: String,
}

impl UserSign {
    pub fn from(user: UserData) -> Self {
        UserSign {
            id: user.id,
            username: user.username,
        }
    }
}
