use tracing::info;

use crate::{backend::{TrainModelDataSend, TrainModelDataRecv}, config::CONFIG, rpc::recommend_algorithm::{RunTrainModelRequestList, RunTrainModelRequest}};


pub mod recommend_algorithm {
    tonic::include_proto!("newsrecommend");
}

impl From<TrainModelDataSend> for recommend_algorithm::RunTrainModelRequest {
    fn from(data: TrainModelDataSend) -> Self {
        Self {
            user_id: data.user_id,
            tag_id: data.tag_id,
            weight: data.weight,
            time: data.time,
        }
    }
}

pub async fn train_model(send_data: Vec<TrainModelDataSend>) -> anyhow::Result<Vec<TrainModelDataRecv>> {
    let rpc_url = format!("http://{}", CONFIG.common.model_addr);
    info!("Send RPC request to {}", rpc_url);
    let mut client = recommend_algorithm::news_recommend_client::NewsRecommendClient::connect(rpc_url).await?;
    let send_data = RunTrainModelRequestList {
        train_model_data: send_data.into_iter().map(|data| data.into()).collect::<Vec<RunTrainModelRequest>>(),
    };
    let recv_data = client.run_train_model(send_data).await?;
    let recv_data = recv_data.into_inner().update_data.into_iter().map(|data| Ok(TrainModelDataRecv {
        user_id: data.user_id,
        tag_id: data.tag_id,
        weight: data.weight,
    })).collect::<anyhow::Result<Vec<TrainModelDataRecv>>>()?;
    Ok(recv_data)
}