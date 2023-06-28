use poem_openapi::payload::Json;

use crate::common::{
    data::{self, DbPool},
    object::news::{DetailResponse, AbstractResponse},
    ApiResult, ErrorMessage, NoData,
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
    let interests = data::tag::find_all_tag_name_by_news_id(pool, news_id).await
        .map_err(|e| crate::common::ApiError::Error(Json(ErrorMessage::new(e))))?;
    data::user::update_interests_by_id(&mut tx, user_id, interests, 4.5, true).await?;

    // 然后去获取新闻详情
    match data::news::find_by_id(pool, news_id).await {
        Ok(news) => {
            tx.commit().await.unwrap();
            Ok(Json(news))
        }
        Err(e) => Err(crate::common::ApiError::Error(Json(ErrorMessage::new(e)))),
    }
}

/// 用户获取新闻列表
pub async fn recommend(pool: &DbPool, user_id: i32) -> ApiResult<Vec<AbstractResponse>> {
    
    todo!()
}