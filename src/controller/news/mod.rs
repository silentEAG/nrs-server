use std::collections::HashSet;

use poem_openapi::payload::Json;
use rand::seq::IteratorRandom;

use crate::{
    common::{
        data::{self, DbPool},
        object::news::{AbstractResponse, DetailResponse, RandomTagResponse},
        ApiResult, ErrorMessage, NoData,
    },
    config::CONFIG,
    rpc::{self, recommend::UserCfRequest},
};

/// 用户点赞新闻
pub async fn like(pool: &DbPool, _user_id: i32, news_id: i32) -> ApiResult<NoData> {
    match data::news::increase_like(pool, news_id).await {
        Ok(_) => Ok(Json(NoData {})),
        Err(e) => Err(crate::common::ApiError::Error(Json(ErrorMessage::new(e)))),
    }
}

/// 用户获取新闻详情
pub async fn get(pool: &DbPool, user_id: i32, news_id: i32) -> ApiResult<DetailResponse> {
    // 事务处理
    let mut tx = pool.begin().await.unwrap();

    // 首先会历史记录中添加一条记录
    if let Err(e) = data::user::update_history(&mut tx, user_id, news_id).await {
        tracing::error!("{}", e);
    }

    // 动态更改 tag 权重
    let interests = data::tag::find_all_tag_name_by_news_id(pool, news_id)
        .await
        .map_err(|e| crate::common::ApiError::Error(Json(ErrorMessage::new(e))))?;
    data::user::update_interests_by_id(&mut tx, user_id, interests, 4.8, true).await?;

    // 然后去获取新闻详情
    match data::news::find_by_id(pool, news_id).await {
        Ok(news) => {
            tx.commit().await.unwrap();
            Ok(Json(news))
        }
        Err(e) => Err(crate::common::ApiError::Error(Json(ErrorMessage::new(e)))),
    }
}

pub async fn get_random_tags(pool: &DbPool, limit: i32) -> ApiResult<RandomTagResponse> {
    let tags = data::tag::find_random_tags_name(pool, limit)
        .await
        .map_err(|e| crate::common::ApiError::Error(Json(ErrorMessage::new(e))))?;
    Ok(Json(RandomTagResponse { tags }))
}

/// 用户获取新闻列表
pub async fn recommend_by_user_ids(
    pool: &DbPool,
    user_ids: Vec<i32>,
    limit: i32,
) -> ApiResult<Vec<AbstractResponse>> {
    // 获取 RPC Client
    let rpc_url = format!("http://{}", CONFIG.common.model_addr);
    let mut rpc_client =
        crate::rpc::recommend::news_recommend_client::NewsRecommendClient::connect(rpc_url)
            .await
            .map_err(|e| crate::common::ApiError::Error(Json(ErrorMessage::new(e))))?;

    // 通过 RPC 获取推荐 tag
    let response = rpc::get_recommend_tags(
        &mut rpc_client,
        UserCfRequest {
            user_id: user_ids,
            num: 5,
        },
    )
    .await;

    let tag_ids = match &response {
        Ok(response) => response.response[0].tag_id.clone(),
        Err(e) => {
            tracing::error!("rpc error: {}", e);
            data::tag::find_random_tags_id(pool, limit).await.unwrap()
        }
    };

    let mut news_vec = Vec::new();

    // 通过 tag 来获取新闻
    for tag_id in tag_ids {
        // tracing::info!("get tag_id: {}", tag_id);
        let news = data::news::find_by_tag_id(pool, tag_id, 100)
            .await
            .map_err(|e| crate::common::ApiError::Error(Json(ErrorMessage::new(e))))?;
        // tracing::info!("news: {:?}", news);
        news_vec.extend(news);
    }

    let news_set_len = news_vec.len();
    let limit = match news_set_len < limit as usize {
        true => news_set_len,
        false => limit as usize,
    };
    
    // 去重处理并随机选择 limit 条新闻出来
    let news_set = news_vec
        .drain(..)
        .collect::<HashSet<AbstractResponse>>()
        .into_iter()
        .choose_multiple(&mut rand::thread_rng(), limit);
    Ok(Json(news_set))
}
