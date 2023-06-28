use crate::common::data::DbPool;



pub async fn get_test_pool() -> DbPool {
    use sqlx::postgres::PgPoolOptions;
    let db_link = "postgres://news_recommender:nekopara@127.0.0.1:5432/news_recommend";
    let pool = PgPoolOptions::new()
        .min_connections(5)
        .max_connections(15)
        .connect(&db_link)
        .await.unwrap();
    pool
}