use super::{DbPool, TransPool};

#[derive(sqlx::FromRow)]
pub struct TagData {
    pub id: i32,
    pub name: String,
}

pub async fn is_exist_by_name(pool: &DbPool, tag_name: String) -> anyhow::Result<bool> {
    let result = sqlx::query("SELECT id FROM tag WHERE tag_name = $1")
        .bind(tag_name)
        .fetch_optional(pool)
        .await?;
    Ok(result.is_some())
}

pub async fn find_by_name(pool: &DbPool, tag_name: String) -> anyhow::Result<TagData> {
    let tag = sqlx::query_as::<_, TagData>("SELECT * FROM tag WHERE tag_name = $1")
        .bind(tag_name)
        .fetch_one(pool)
        .await?;
    Ok(tag)
}

pub async fn find_by_id(pool: &DbPool, tag_id: i32) -> anyhow::Result<TagData> {
    let tag = sqlx::query_as::<_, TagData>("SELECT * FROM tag WHERE id = $1")
        .bind(tag_id)
        .fetch_one(pool)
        .await?;
    Ok(tag)
}

pub async fn insert(pool: &mut TransPool<'_>, tag_name: &String) -> anyhow::Result<()> {
    let _ = sqlx::query(
        "
        INSERT INTO tag (name) VALUES ($1)
        ON CONFLICT (name) DO NOTHING;",
    )
    .bind(tag_name)
    .execute(pool)
    .await?;
    Ok(())
}

/// 随机返回一些 tag
pub async fn find_random_tags_name(pool: &DbPool, limit: i32) -> anyhow::Result<Vec<String>> {
    let tags = sqlx::query_as::<_, TagData>("SELECT id, name FROM tag ORDER BY random() LIMIT $1")
        .bind(limit)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|tag| tag.name)
        .collect::<Vec<String>>();
    Ok(tags)
}

pub async fn find_random_tags_id(pool: &DbPool, limit: i32) -> anyhow::Result<Vec<i32>> {
    let tags = sqlx::query_as::<_, TagData>("SELECT id, name FROM tag ORDER BY random() LIMIT $1")
        .bind(limit)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|tag| tag.id)
        .collect::<Vec<i32>>();
    Ok(tags)
}

/// 通过 news_id 查找 tag_name
pub async fn find_all_tag_name_by_news_id(
    pool: &DbPool,
    news_id: i32,
) -> anyhow::Result<Vec<String>> {
    let result = sqlx::query_as::<_, (String,)>("SELECT tag_name FROM news_tag WHERE news_id = $1")
        .bind(news_id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|tag_name| tag_name.0)
        .collect::<Vec<String>>();
    Ok(result)
}
