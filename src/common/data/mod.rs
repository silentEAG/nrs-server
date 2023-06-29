// 定义数据库的表结构

use sqlx::{Postgres, Transaction};
use tonic::transport::Channel;

use crate::rpc::recommend::news_recommend_client::NewsRecommendClient;

pub type DbPool = sqlx::PgPool;

pub type TransPool<'c> = Transaction<'c, Postgres>;
pub type RpcClient = NewsRecommendClient<Channel>;

pub mod news;
pub mod tag;
pub mod user;
