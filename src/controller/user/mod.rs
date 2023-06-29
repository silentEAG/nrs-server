use std::collections::HashSet;

use jwt::SignWithKey;
use poem_openapi::payload::Json;
use rand::seq::IteratorRandom;
use tracing::debug;

use crate::{
    common::{
        data::{self, DbPool},
        object::{
            self,
            user::{LoginRequest, LoginSuccess, RegisterRequest, UserSign},
        },
        ApiError, ApiResult, ErrorMessage, NoData,
    },
    config::{ServerKey, CONFIG},
    rpc::{
        self,
        recommend::{news_recommend_client::NewsRecommendClient, ItemCfRequest},
    },
    util::calc_password_hash,
};

/// 用户注册操作
pub async fn register(pool: &DbPool, user: RegisterRequest) -> ApiResult<NoData> {
    let username = user.username;
    debug!("{} start to register", username);

    // 先判断用户是否存在
    match data::user::is_exist_by_username(pool, username.clone()).await {
        Ok(result) if result => {
            return Err(ApiError::UserAlreadyExists);
        }
        Err(e) => {
            return Err(ApiError::DBError(Json(ErrorMessage::new(e))));
        }
        _ => {}
    }

    // 然后再插入新用户
    // 使用 salt 计算 hash 后的 password
    let password_hash = calc_password_hash(&user.password, &username);
    // debug!("password hash: {}", password);

    // 判断用户性别，man、woman、unknown
    let sex = match user.sex {
        Some(s) if s == "man" => "man",
        Some(s) if s == "woman" => "woman",
        _ => "unknown",
    }
    .to_string();

    // 判断用户年龄
    let age = user.age;

    // 执行插入操作
    debug!("{} is finishing register", username);
    match data::user::insert_new_user(pool, username, password_hash, sex, age).await {
        Ok(_) => Ok(Json(NoData {})),
        Err(e) => Err(ApiError::DBError(Json(ErrorMessage::new(e)))),
    }
}

/// 用户登录操作
pub async fn login(
    pool: &DbPool,
    server_key: &ServerKey,
    user: LoginRequest,
) -> ApiResult<LoginSuccess> {
    // 使用 salt 计算 hash 后的 password
    let password_hash = calc_password_hash(&user.password, &user.username);

    let user = match data::user::find_by_name(pool, user.username).await {
        Ok(user) => user,
        Err(_) => {
            return Err(ApiError::UserNotExists);
        }
    };

    // 密码错误
    if user.password != password_hash {
        return Err(ApiError::UserPasswordError);
    }

    // 密码正确返回 token
    match UserSign::from(user).sign_with_key(server_key) {
        Ok(token) => Ok(Json(LoginSuccess { token })),
        Err(e) => Err(ApiError::SignError(Json(ErrorMessage::new(e.to_string())))),
    }
}

/// 获取用户信息
pub async fn get_info(pool: &DbPool, user_id: i32) -> ApiResult<object::user::InfoResponse> {
    // 获取用户 meta 信息
    let user = match data::user::find_by_id(pool, user_id).await {
        Ok(user) => user,
        Err(_) => {
            return Err(ApiError::UserNotExists);
        }
    };

    // 获取用户兴趣 tags
    let interests = match data::user::get_interests_by_user_id(pool, user_id).await {
        Ok(interests) => interests,
        Err(e) => {
            return Err(ApiError::TagQueryError(Json(ErrorMessage::new(
                e.to_string(),
            ))));
        }
    };

    // 组合返回
    Ok(Json(object::user::InfoResponse {
        username: user.username,
        interests,
        age: user.age,
        sex: user.sex,
        create_time: user.create_time,
    }))
}

/// 获取用户历史记录
pub async fn get_history(pool: &DbPool, user_id: i32) -> ApiResult<object::user::HistoryResponse> {
    // 通过 user_id 获取用户历史记录
    let history = match data::user::get_history_by_user_id(pool, user_id).await {
        Ok(history) => history,
        Err(e) => {
            return Err(ApiError::DBError(Json(ErrorMessage::new(e))));
        }
    };

    Ok(Json(object::user::HistoryResponse { news: history }))
}

/// 更新用户信息
/// 1. 更新密码
/// 2. 更新兴趣 tag （注：这里的 tag 更新是表示对这个 tag 感兴趣，将 weight 增加到 5）
pub async fn update(
    pool: &DbPool,
    user_id: i32,
    user_update: object::user::UpdateRequest,
) -> ApiResult<NoData> {
    // 通过 user_id 获取用户信息
    let user = match data::user::find_by_id(pool, user_id).await {
        Ok(user) => user,
        Err(e) => {
            return Err(ApiError::DBError(Json(ErrorMessage::new(e))));
        }
    };

    // 事务开始
    let mut tx = pool.begin().await.unwrap();

    // 更新兴趣 tag（即表示对这个 tag 感兴趣）
    if let Some(interests) = user_update.interests {
        data::user::update_interests_by_id(&mut tx, user_id, interests, 5.0, true).await?;
    }

    // 更新密码
    if let Some(password) = user_update.password {
        let password_hash = calc_password_hash(&password, &user.username);
        data::user::update_password_by_id(&mut tx, user_id, password_hash)
            .await
            .map_err(|e| ApiError::UserUpdateFailed(Json(ErrorMessage::new(e.to_string()))))?;
    }

    tx.commit().await.unwrap();
    Ok(Json(NoData {}))
}

/// 通过用户 id 的兴趣 tag 来推送相关用户
pub async fn connect(pool: &DbPool, user_id: i32, limit: i32) -> ApiResult<Vec<UserSign>> {
    let tag_ids = data::user::get_tag_id_by_user_id(pool, user_id, -1.0)
        .await
        .map_err(|e| ApiError::DBError(Json(ErrorMessage::new(e))))?;

    if tag_ids.len() == 0 {
        return Err(ApiError::NoRecommendUserFound);
    }

    // 获取 RPC Client
    let rpc_url = format!("http://{}", CONFIG.common.model_addr);
    let mut rpc_client = NewsRecommendClient::connect(rpc_url)
        .await
        .map_err(|e| crate::common::ApiError::Error(Json(ErrorMessage::new(e))))?;


    let response = rpc::get_recommend_users(
        &mut rpc_client,
        ItemCfRequest {
            tag_id: tag_ids,
            num: limit,
        },
    )
    .await
    .map_err(|e| crate::common::ApiError::RPCError(Json(ErrorMessage::new(e))))?;

    let mut users = Vec::new();

    for item in response.response {
        for user_id in item.user_id {
            let user = data::user::find_by_id(pool, user_id)
                .await
                .map_err(|e| ApiError::DBError(Json(ErrorMessage::new(e))))?;
            users.push(UserSign::from(user));
        }
    }

    Ok(Json(
        users
            .drain(..)
            .collect::<HashSet<UserSign>>()
            .into_iter()
            .choose_multiple(&mut rand::thread_rng(), limit as usize),
    ))
}
