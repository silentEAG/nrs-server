use poem_openapi::payload::Json;

use crate::common::{
    data::{self, user::UserData, DbPool},
    object, ApiError, ApiResult, ErrorMessage,
};

// Admin 获取用户信息
pub async fn get_user_by_id(pool: &DbPool, user_id: i32) -> ApiResult<object::user::InfoResponse> {
    super::user::get_info(pool, user_id).await
}
