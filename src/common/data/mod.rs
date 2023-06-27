// 定义数据库的表结构

pub type DbPool = sqlx::PgPool;
pub type TransPool = sqlx::pool::PoolConnection<sqlx::Postgres>;

pub mod news;
pub mod user;
