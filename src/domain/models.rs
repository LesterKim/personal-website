#[derive(Clone)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price_range: String,
} 