use serde::{Serialize, Deserialize};
use cfg_if::cfg_if;
use crate::models::category::Category;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub hero_image: String,
    pub content: String,
    pub published_at: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<Category>,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use femark::HTMLOutput;
        use chrono::{DateTime, Local};

        #[derive(sqlx::FromRow, sqlx::Type)]
        pub struct SqlPost{
            pub id: i64,
            pub title: String,
            pub description: String,
            pub hero_image: String,
            pub content: String,
            pub published_at: DateTime<Local>,
            pub slug: String,
            pub categories: Vec<Category>,
        }

        impl SqlPost {
            pub fn into_post(self) -> BlogPost {
                let HTMLOutput{content,..} = femark::process_markdown_to_html(&self.content).unwrap_or_default();
                BlogPost {
                    id: self.id,
                    title: self.title,
                    description: self.description,
                    hero_image: self.hero_image,
                    published_at: self.published_at.format("%d/%m/%Y").to_string(),
                    content,
                    slug: self.slug,
                    categories: self.categories,
                }
            }
        }
    }
}
