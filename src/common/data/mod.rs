// 定义数据库的表结构

use sqlx::{Postgres, Transaction};

pub type DbPool = sqlx::PgPool;

pub type TransPool<'c> = Transaction<'c, Postgres>;

pub mod news;
pub mod tag;
pub mod user;
