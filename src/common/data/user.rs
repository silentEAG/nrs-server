use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{common::{object, ApiResult, NoData}, backend::TrainModelDataSend};

use super::{DbPool, TransPool};

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
    pool: &mut TransPool<'_>,
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
    pool: &mut TransPool<'_>,
    user_id: i32,
    interests: Vec<String>,
    weight: f64,
    change_time: bool
) -> ApiResult<NoData> {
    let mut error_array = Vec::new();

    for interest in interests {
        // 首先更新 tag 表，如果没有便插入
        if let Err(e) = super::tag::insert(&mut *pool, &interest).await {
            tracing::error!("{}", e);
        }

        let sql_query = match change_time {
            true => {
                "
                INSERT INTO interest (user_id, news_tag, weight) 
                VALUES ($1, $2, $3) 
                ON CONFLICT (user_id, news_tag) DO 
                    UPDATE SET 
                    weight = CASE WHEN interest.weight < $3 THEN $3 ELSE interest.weight END
                    last_view_time = now()
                "
            },
            false => {
                "
                INSERT INTO interest (user_id, news_tag, weight) 
                VALUES ($1, $2, $3) 
                ON CONFLICT (user_id, news_tag) DO 
                    UPDATE SET 
                    weight = CASE WHEN interest.weight < $3 THEN $3 ELSE interest.weight END
                "
            }
        };

        // 再更新 interest 表，与 user 相关性大
        let result = sqlx::query(sql_query)
            .bind(user_id)
            .bind(&interest)
            .bind(weight)
            .execute(&mut *pool)
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

/// 通过用户 id 获取用户兴趣 tag name
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


pub async fn get_train_model_data(pool: &DbPool) -> anyhow::Result<Vec<TrainModelDataSend>> {
    let result = sqlx::query_as::<_, (i32, i32, f64, chrono::NaiveDateTime)>("
        SELECT interest.user_id, tag.id as tag_id, interest.weight, interest.last_view_time as time
        FROM interest, tag
        WHERE tag.name = interest.news_tag")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|(user_id, tag_id, weight, time)| {
            TrainModelDataSend {
                user_id,
                tag_id,
                weight,
                time: time.timestamp(),
            }
        })
        .collect::<Vec<TrainModelDataSend>>();
    Ok(result)
}

pub async fn get_history_by_user_id(
    pool: &DbPool,
    user_id: i32,
) -> anyhow::Result<Vec<object::news::AbstractResponse>> {
    let mut historys = sqlx::query_as::<_, object::news::AbstractResponse>("
        SELECT news.id as news_id, news.title, news.abstracts, news.source, news.create_time, news.likes as like, array_agg(news_tag.tag_name) as tags
        FROM history, news
        JOIN news_tag ON news_tag.news_id = news.id
        WHERE  history.user_id = $1 AND history.news_id = news.id
        GROUP BY news.id
        LIMIT 50")
        .bind(user_id)
        .fetch_all(pool)
        .await?;
    historys.sort_by(|a, b| {
        b.create_time
            .partial_cmp(&a.create_time)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Ok(historys)
}

pub async fn update_history(
    pool: &mut TransPool<'_>,
    user_id: i32,
    news_id: i32,
) -> anyhow::Result<()> {
    let _ = sqlx::query("INSERT INTO history (user_id, news_id) VALUES ($1, $2) ON CONFLICT (user_id, news_id) DO UPDATE SET last_view_time = now()")
        .bind(user_id)
        .bind(news_id)
        .execute(pool)
        .await?;
    Ok(())
}


#[tokio::test]
async fn test_send_train_model() {
    use sqlx::postgres::PgPoolOptions;
    let db_link = "postgres://news_recommender:nekopara@127.0.0.1:5432/news_recommend";
    let pool = PgPoolOptions::new()
        .min_connections(5)
        .max_connections(15)
        .connect(&db_link)
        .await.unwrap();
    let res = get_train_model_data(&pool).await.unwrap();
    println!("{:?}", res);
}
