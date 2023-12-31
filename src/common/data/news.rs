use crate::common::object::news::{AbstractResponse, DetailResponse};

use super::{DbPool, TransPool};

/// 插入一条新闻
/// - 新闻的 abstracts 为唯一键，若违反约束便停止插入
pub async fn insert_new_news(
    pool: &mut TransPool<'_>,
    title: String,             // 新闻标题
    content: String,           // 新闻主要内容
    abstracts: Option<String>, // 新闻摘要，如果没有便选取内容的前100个字
    source: String,            // 新闻来源
    tags: Vec<String>,         // 新闻 tag
    link: String,              // 新闻原链接
) -> anyhow::Result<()> {
    let abstracts = match abstracts {
        Some(abstracts) => abstracts,
        // 如果没有摘要，就取前 100 个字符
        None => content.chars().take(100).collect::<String>(),
    };

    tracing::info!("insert news: {}", title);

    // 插入 news 表
    let (news_id, ) =
        sqlx::query_as::<_, (i32, )>("INSERT INTO news (title, content, abstracts, source, link, likes) VALUES ($1, $2, $3, $4, $5, 0) RETURNING id")
            .bind(title)
            .bind(content)
            .bind(abstracts)
            .bind(source)
            .bind(link)
            .fetch_one(&mut *pool)
            .await
            .map_err(|e| {
                tracing::error!("{}", e);
                e
            })?;

    tracing::info!("get news_id: {}", news_id);

    // 更新 tag 相关的表
    for tag in tags {
        // 首先更新 tag 表，如果没有便插入
        if let Err(e) = super::tag::insert(&mut *pool, &tag).await {
            tracing::error!("{}", e);
        }

        // 然后更新 news_tag 表，与 news 相关性大
        if let Err(e) = update_news_tag(&mut *pool, news_id, tag).await {
            tracing::error!("{}", e);
        }
    }
    Ok(())
}

pub async fn update_news_tag(
    pool: &mut TransPool<'_>,
    news_id: i32,
    tag: String,
) -> anyhow::Result<()> {
    let _ = sqlx::query("INSERT INTO news_tag (tag_name, news_id) VALUES ($1, $2) ON CONFLICT (tag_name, news_id) DO NOTHING")
        .bind(tag)
        .bind(news_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn increase_like(pool: &DbPool, news_id: i32) -> anyhow::Result<()> {
    let _ = sqlx::query("UPDATE news SET likes = likes + 1 WHERE id = $1")
        .bind(news_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn find_by_id(pool: &DbPool, news_id: i32) -> anyhow::Result<DetailResponse> {
    let news = sqlx::query_as::<_, DetailResponse>(
        "SELECT news.id as news_id, news.title, news.content, news.source, news.create_time, news.likes as like, array_agg(news_tag.tag_name) as tags
        FROM news 
        LEFT JOIN news_tag 
        ON news.id = news_tag.news_id 
        WHERE news.id = $1 
        GROUP BY news.id")
        .bind(news_id)
        .fetch_one(pool)
        .await?;
    Ok(news)
}

pub async fn find_by_id_abstract(pool: &DbPool, news_id: i32) -> anyhow::Result<AbstractResponse> {
    let news = sqlx::query_as::<_, AbstractResponse>(
        "SELECT news.id as news_id, news.title, news.abstracts, news.source, news.create_time, news.likes as like, array_agg(news_tag.tag_name) as tags
        FROM news 
        LEFT JOIN news_tag 
        ON news.id = news_tag.news_id 
        WHERE news.id = $1 
        GROUP BY news.id")
        .bind(news_id)
        .fetch_one(pool)
        .await?;
    Ok(news)
}

pub async fn find_by_tag_id(pool: &DbPool, tag_id: i32, per_limit: i32) -> anyhow::Result<Vec<AbstractResponse>> {
    let news = sqlx::query_as::<_, AbstractResponse>(
        "
        SELECT news.id as news_id, news.title, news.abstracts, news.source, news.create_time, news.likes as like, array_agg(news_tag.tag_name) as tags
        FROM news 
        LEFT JOIN news_tag 
        ON news.id = news_tag.news_id 
        WHERE news.id IN 
        (
            SELECT DISTINCT news_tag.news_id
            FROM news_tag 
            WHERE news_tag.tag_name in (SELECT T.name FROM tag AS T WHERE T.id = $1)
        )
        GROUP BY news.id 
        ORDER BY RANDOM()
        LIMIT $2",
    )
    .bind(tag_id)
    .bind(per_limit)
    .fetch_all(pool)
    .await?;

    Ok(news)
}

// pub async fn find_by_tag_id(pool: &DbPool, tag_id: i32) -> anyhow::Result<Vec<AbstractResponse>> {
//     let news = sqlx::query_as::<_, (i32,)>(
//         "SELECT DISTINCT news_tag.news_id
//         FROM news_tag
//         WHERE news_tag.tag_name in (SELECT T.name FROM tag AS T WHERE T.id = $1)",
//     )
//     .bind(tag_id)
//     .fetch_all(pool)
//     .await?
//     .into_iter()
//     .map(|(news_id,)| news_id);

//     let mut result = Vec::new();

//     for news_id in news {
//         let news = find_by_id_abstract(pool, news_id).await?;
//         result.push(news);
//     }

//     Ok(result)
// }
