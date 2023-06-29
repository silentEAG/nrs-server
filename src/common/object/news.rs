use poem_openapi::Object;

#[derive(Object)]
pub struct CreateNewsRequest {
    pub title: String,             // 新闻标题
    pub content: String,           // 新闻内容
    pub source: String,            // 新闻来源
    pub abstracts: Option<String>, // 新闻摘要
    pub link: String,              // 新闻链接
    pub tags: Vec<String>,         // 新闻 tag
}

#[derive(Object, sqlx::FromRow, PartialEq, Eq, Hash, Debug)]
pub struct AbstractResponse {
    pub news_id: i32,
    pub title: String,
    pub abstracts: String,
    pub source: String,
    pub create_time: chrono::NaiveDateTime,
    pub like: i32,
    pub tags: Vec<String>,
}

#[derive(Object, sqlx::FromRow)]
pub struct DetailResponse {
    pub news_id: i32,
    pub title: String,
    pub content: String,
    pub source: String,
    pub create_time: chrono::NaiveDateTime,
    pub like: i32,
    pub tags: Option<Vec<String>>,
}

#[derive(Object)]
pub struct RandomTagResponse {
    pub tags: Vec<String>,
}
