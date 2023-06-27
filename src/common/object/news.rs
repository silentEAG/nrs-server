use poem_openapi::Object;

#[derive(Object)]
pub struct CreateNewsRequest {
    pub title: String,
    pub content: String,
    pub source: String,
    pub tags: Vec<String>,
}

#[derive(Object)]
pub struct AbstractResponse {
    pub news_id: i32,
    pub title: String,
    pub abstracts: String,
    pub source: String,
    pub create_time: String,
    pub like: i32,
    pub tags: Vec<String>,
}

#[derive(Object)]
pub struct DetailResponse {
    pub news_id: i32,
    pub title: String,
    pub content: String,
    pub source: String,
    pub create_time: String,
    pub like: i32,
    pub tags: Vec<String>,
}
