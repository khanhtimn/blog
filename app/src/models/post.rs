use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub category: String,
    pub hero_image: String,
    pub content: String,
    pub published_at: DateTime<Local>,
    pub slug: String,
}
