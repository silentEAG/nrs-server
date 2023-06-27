use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::common::{object, ApiResult, NoData};

use super::DbPool;

#[derive(Debug, sqlx::FromRow)]
pub struct UserData {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub sex: String,
    pub age: i32,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
}

/// 通过用户名判断用户是否存在
pub async fn is_exist_by_username(pool: &DbPool, username: String) -> anyhow::Result<bool> {
    let result = sqlx::query("SELECT id FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    Ok(result.is_some())
}

/// 新增用户
pub async fn insert_new_user(
    pool: &DbPool,
    username: String,
    password: String,
    sex: String,
    age: i32,
) -> anyhow::Result<()> {
    let result =
        sqlx::query("INSERT INTO users (username, password, sex, age) VALUES ($1, $2, $3, $4)")
            .bind(username)
            .bind(password)
            .bind(sex)
            .bind(age)
            .execute(pool)
            .await;

    match result {
        Ok(r) if r.rows_affected() == 1 => Ok(()),
        _ => Err(anyhow::anyhow!("新增用户失败")),
    }
}

/// 通过用户名查找用户
pub async fn find_by_name(pool: &DbPool, username: String) -> anyhow::Result<UserData> {
    let user = sqlx::query_as::<_, UserData>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

/// 通过用户 id 查找用户
pub async fn find_by_id(pool: &DbPool, user_id: i32) -> anyhow::Result<UserData> {
    let user = sqlx::query_as::<_, UserData>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

/// 更新用户密码
pub async fn update_password_by_id(
    pool: &DbPool,
    user_id: i32,
    password: String,
) -> anyhow::Result<()> {
    let result = sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
        .bind(password)
        .bind(user_id)
        .execute(pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() == 1 => Ok(()),
        _ => Err(anyhow::anyhow!("更新用户密码失败")),
    }
}

/// 更新用户 tags 信息
pub async fn update_interests_by_id(
    pool: &DbPool,
    user_id: i32,
    interests: Vec<String>,
) -> ApiResult<NoData> {
    let mut error_array = Vec::new();

    for interest in interests {
        // 默认插入权重为 100
        let result = sqlx::query(
            "INSERT INTO interest (user_id, news_tag, weight) 
                VALUES ($1, $2, 100) 
                ON CONFLICT (user_id, news_tag) 
                DO UPDATE SET weight = interest.weight + 1",
        )
        .bind(user_id)
        .bind(&interest)
        .execute(pool)
        .await;

        match result {
            Ok(r) if r.rows_affected() == 1 => (),
            Err(e) => {
                error_array.push(interest);
                error!("更新用户兴趣标签失败: {}", e);
            }
            _ => error_array.push(interest),
        }
    }

    match error_array.len() {
        // 如果全部成功，返回 200
        0 => Ok(Json(NoData {})),
        // 如果部分成功，返回 856，并返回未更新成功的标签
        _ => Err(crate::common::ApiError::TagUpdateError(Json(error_array))),
    }
}

/// 通过用户 id 获取用户兴趣 tag
pub async fn get_interests_by_user_id(pool: &DbPool, user_id: i32) -> anyhow::Result<Vec<String>> {
    let result = sqlx::query_as::<_, (String,)>("SELECT news_tag FROM interest WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|(interest,)| interest)
        .collect::<Vec<String>>();
    Ok(result)
}

pub async fn get_history_by_user_id(
    pool: &DbPool,
    user_id: i32,
) -> anyhow::Result<Vec<object::news::AbstractResponse>> {
    // let result = sqlx::query("SELECT ")
    todo!()
}
