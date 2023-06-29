use crate::common::data::RpcClient;

use self::recommend::{
    GetWeightRequest, GetWeightResponse, ItemCfRequest, ItemCfResponse, TrainModelRequest,
    UserCfRequest, UserCfResponse,
};

pub mod recommend {
    tonic::include_proto!("newsrecommend");
}

pub async fn get_weight(
    client: &mut RpcClient,
    request: GetWeightRequest,
) -> anyhow::Result<GetWeightResponse> {
    for x in &request.request {
        tracing::info!("GetWeight user_id: {} - tag_id: {} - weight: {} - time: {}", x.user_id, x.tag_id, x.rating, x.last_view_time);
    }
    Ok(client.get_weight(request).await?.into_inner())
}

pub async fn train_model(client: &mut RpcClient, request: TrainModelRequest) -> anyhow::Result<()> {
    client.train_model(request).await?;
    Ok(())
}

pub async fn get_recommend_tags(
    client: &mut RpcClient,
    request: UserCfRequest,
) -> anyhow::Result<UserCfResponse> {
    tracing::info!("{:?}", request);
    Ok(client.get_recommend_tags(request).await?.into_inner())
}

pub async fn get_recommend_users(
    client: &mut RpcClient,
    request: ItemCfRequest,
) -> anyhow::Result<ItemCfResponse> {
    Ok(client.get_recommend_users(request).await?.into_inner())
}
