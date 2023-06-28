use poem_openapi::payload::Json;

use crate::common::{
    data::{self, DbPool},
    object, ApiError, ApiResult, ErrorMessage, NoData,
};

/// Admin 获取用户信息
pub async fn get_user_by_id(pool: &DbPool, user_id: i32) -> ApiResult<object::user::InfoResponse> {
    // 直接调用 user 模块的相同方法
    super::user::get_info(pool, user_id).await
}

/// 新建新闻到数据库中
pub async fn create_news(
    pool: &DbPool,
    news: object::news::CreateNewsRequest,
) -> ApiResult<NoData> {
    let mut tx = pool.begin().await.unwrap();

    match data::news::insert_new_news(
        &mut tx,
        news.title,
        news.content,
        news.abstracts,
        news.source,
        news.tags,
        news.link
    )
    .await
    {
        Ok(_) => {
            tx.commit().await.unwrap();
            Ok(Json(NoData {}))
        }
        Err(e) => Err(ApiError::DBError(Json(ErrorMessage::new(e)))),
    }
}
