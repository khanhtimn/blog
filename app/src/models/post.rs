use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use crate::models::category::Category;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow, sqlx::Type))]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub hero_image: String,
    pub content: String,
    pub published_at: DateTime<Local>,
    pub slug: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<Category>,
}
