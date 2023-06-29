use tonic::transport::Channel;

use crate::{
    common::data::{self, DbPool, RpcClient},
    config::CONFIG,
    rpc::{
        self,
        recommend::{self, news_recommend_client::NewsRecommendClient},
    },
};

#[derive(sqlx::FromRow, Debug)]
pub struct TrainModelDataSend {
    pub user_id: i32,
    pub tag_id: i32,
    pub weight: f64,
}

#[derive(sqlx::FromRow, Debug)]
pub struct GetWeightDataSend {
    pub user_id: i32,
    pub tag_id: i32,
    pub weight: f64,
    pub time: i64,
}

#[derive(sqlx::FromRow, Debug)]
pub struct GetWeightlDataRecv {
    pub user_id: i32,
    pub tag_id: i32,
    pub weight: f64,
}

pub async fn train_model(pool: &DbPool, mut client: RpcClient) -> anyhow::Result<()> {
    // 准备训练数据
    let train_model_data_send = data::user::get_train_model_data(&pool).await?;
    // 发送 rpc 请求
    rpc::train_model(&mut client, train_model_data_send).await?;
    Ok(())
}

pub async fn update_weight(pool: &DbPool, mut client: RpcClient) -> anyhow::Result<()> {
    // 准备训练数据
    let train_model_data_send = data::user::update_weight_data(&pool).await?;
    // 发送 rpc 请求
    let response = rpc::get_weight(&mut client, train_model_data_send).await?;

    // 开启事务
    let mut tx = pool.begin().await?;

    // 更新数据库
    for interest in response.response {
        let tag = data::tag::find_by_id(&pool, interest.tag_id).await?;
        // tracing::info!("update insterest: {} - {} - {}", interest.tag_id, tag.name, interest.weight);
        if let Err(_) = data::user::update_interests_by_id(
            &mut tx,
            interest.user_id,
            vec![tag.name],
            interest.weight,
            false,
        )
        .await
        {
            tracing::error!("update interest error");
        }
    }

    // 提交事务
    tx.commit().await?;
    Ok(())
}

/// 处理后台任务函数
pub async fn start(pool: DbPool) -> anyhow::Result<()> {
    let rpc_url = format!("http://{}", CONFIG.common.model_addr);
    let client = recommend::news_recommend_client::NewsRecommendClient::connect(rpc_url).await?;

    tokio::spawn(async move {
        tokio::join!(
            async {
                loop {
                    // 每次执行训练模型
                    tracing::info!("train model task start");
                    if let Err(e) = train_model(&pool, client.clone()).await {
                        tracing::error!("train model task error: {}", e);
                    }
                    tracing::info!("train model task finish");
                    // 2 min 执行一次
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                }
            },
            async {
                loop {
                    // 每次执行更新权重
                    tracing::info!("update weight task start");
                    if let Err(e) = update_weight(&pool, client.clone()).await {
                        tracing::error!("update weight task error: {}", e);
                    }
                    tracing::info!("update weight task finish");
                    // 30s 执行一次
                    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                }
            }
        );
    });

    Ok(())
}

#[tokio::test]
async fn test_back_task() {
    let pool = crate::test::get_test_pool().await;
    start(pool).await.unwrap();
}
