use super::DbPool;

pub async fn insert_new_news(
    pool: &DbPool,
    title: String,
    content: String,
    abstracts: Option<String>,
    source: String,
) -> anyhow::Result<()> {
    let abstracts = match abstracts {
        Some(abstracts) => abstracts,
        // 如果没有摘要，就取前 100 个字符
        None => content.chars().take(100).collect::<String>(),
    };

    let _ =
        sqlx::query("INSERT INTO news (title, content, abstracts, source) VALUES ($1, $2, $3, $4)")
            .bind(title)
            .bind(content)
            .bind(abstracts)
            .bind(source)
            .execute(pool)
            .await?;

    Ok(())
}

pub async fn update_news_tags(
    pool: &DbPool,
    news_id: i32,
    tags: Vec<String>,
) -> anyhow::Result<()> {
    todo!();
    Ok(())
}
