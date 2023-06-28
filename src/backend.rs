use crate::{common::data::{DbPool, self}, rpc};



#[derive(sqlx::FromRow, Debug)]
pub struct TrainModelDataSend {
    pub user_id: i32,
    pub tag_id: i32,
    pub weight: f64,
    pub time: i64
}


#[derive(sqlx::FromRow, Debug)]
pub struct TrainModelDataRecv {
    pub user_id: i32,
    pub tag_id: i32,
    pub weight: f64,
}

pub async fn train_model(pool: &DbPool) -> anyhow::Result<()> {
    // 准备训练数据
    let train_model_data_send = data::user::get_train_model_data(pool).await?;

    // 发送 rpc 请求
    let train_model_data_recv = rpc::train_model(train_model_data_send).await?;

    let mut tx = pool.begin().await?;

    // 更新数据库
    for interest in train_model_data_recv {
        let tag = data::tag::find_by_id(pool, interest.tag_id).await?;
        if let Err(_) = data::user::update_interests_by_id(&mut tx, interest.user_id, vec![tag.name], interest.weight, false).await {

        }
    }

    tx.commit().await?;
    Ok(())
}

/// 处理后台任务函数
pub async fn start(pool: DbPool) -> anyhow::Result<()> {
    // train model task
    tokio::spawn(async move {
        loop {
            // 每次执行训练模型
            tracing::info!("train model task start");
            train_model(&pool).await.unwrap();
            // 2 min 执行一次
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 2)).await;   
        }
    });
    Ok(())
}

#[tokio::test]
async fn test_back_task() {
    let pool = crate::test::get_test_pool().await;
    start(pool).await.unwrap();
}