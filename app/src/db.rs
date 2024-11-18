use sqlx::PgPool;
use crate::models::{category::Category, post::BlogPost, post::SqlPost};


#[derive(Debug, Clone)]
pub struct PostRepository(PgPool);

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    pub async fn get_all_categories(&self) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, slug, description
            FROM categories
            ORDER BY name ASC
            "#
        )
        .fetch_all(&self.0)
        .await
    }

    pub async fn create_category(&self, category: &Category) -> Result<Category, sqlx::Error> {
        sqlx::query_as!(
            Category,
            r#"
            INSERT INTO categories (name, slug, description)
            VALUES ($1, $2, $3)
            RETURNING id, name, slug, description
            "#,
            category.name,
            category.slug,
            category.description
        )
        .fetch_one(&self.0)
        .await
    }

    pub async fn get_all_posts_with_categories(&self) -> Result<Vec<BlogPost>, sqlx::Error> {
        let posts = sqlx::query_as!(
            SqlPost,
            r#"
            SELECT
                p.id, p.title, p.description, p.hero_image,
                p.content, p.published_at, p.slug,
                ARRAY[]::TEXT[] as "categories!: Vec<Category>"
            FROM blog_posts p
            ORDER BY p.published_at DESC
            "#
        )
        .fetch_all(&self.0)
        .await?;

        let mut posts_with_categories = vec![];

        for mut post in posts {
            let categories = sqlx::query_as!(
                Category,
                r#"
                SELECT c.id, c.name, c.slug, c.description
                FROM categories c
                JOIN blog_posts_categories pc ON c.id = pc.category_id
                WHERE pc.blog_post_id = $1
                ORDER BY c.name
                "#,
                post.id
            )
            .fetch_all(&self.0)
            .await?;

            post.categories = categories;
            posts_with_categories.push(post.into_post());
        }

        Ok(posts_with_categories)
    }

    pub async fn search_posts_by_category(
        &self,
        category_slug: &str
    ) -> Result<Vec<BlogPost>, sqlx::Error> {
        let posts = sqlx::query_as!(
            SqlPost,
            r#"
            SELECT DISTINCT
                p.id, p.title, p.description, p.hero_image,
                p.content, p.published_at, p.slug,
                ARRAY[]::TEXT[] as "categories!: Vec<Category>"
            FROM blog_posts p
            JOIN blog_posts_categories pc ON p.id = pc.blog_post_id
            JOIN categories c ON pc.category_id = c.id
            WHERE c.slug = $1
            ORDER BY p.published_at DESC
            "#,
            category_slug
        )
        .fetch_all(&self.0)
        .await?;

        let mut posts_with_categories = vec![];

        for mut post in posts {
            let categories = sqlx::query_as!(
                Category,
                r#"
                SELECT c.id, c.name, c.slug, c.description
                FROM categories c
                JOIN blog_posts_categories pc ON c.id = pc.category_id
                WHERE pc.blog_post_id = $1
                ORDER BY c.name
                "#,
                post.id
            )
            .fetch_all(&self.0)
            .await?;

            post.categories = categories;
            posts_with_categories.push(post.into_post());
        }

        Ok(posts_with_categories)
    }

    // pub async fn create_post_with_categories(
    //     &self,
    //     post: &BlogPost,
    //     category_ids: &[i64],
    // ) -> Result<BlogPost, sqlx::Error> {
    //     let mut tx = pool.begin().await?;

    //     // Insert the blog post
    //     let post = sqlx::query_as!(
    //         BlogPost,
    //         r#"
    //         INSERT INTO blog_posts (title, description, hero_image, content, published_at, slug)
    //         VALUES ($1, $2, $3, $4, $5, $6)
    //         RETURNING id, title, description, hero_image, content, published_at, slug,
    //                   ARRAY[]::TEXT[] as "categories!: Vec<Category>"
    //         "#,
    //         post.title,
    //         post.description,
    //         post.hero_image,
    //         post.content,
    //         post.published_at,
    //         post.slug
    //     )
    //     .fetch_one(&mut *tx)
    //     .await?;

    //     // Insert category associations
    //     for &category_id in category_ids {
    //         sqlx::query!(
    //             r#"
    //             INSERT INTO blog_posts_categories (blog_post_id, category_id)
    //             VALUES ($1, $2)
    //             "#,
    //             post.id,
    //             category_id
    //         )
    //         .execute(&mut *tx)
    //         .await?;
    //     }

    //     tx.commit().await?;

    //     // Fetch the post with categories
    //     self.get_post_by_slug(&post.slug).await
    // }

    pub async fn get_post_by_slug(&self, slug: &str) -> Result<BlogPost, sqlx::Error> {
        let mut post = sqlx::query_as!(
            SqlPost,
            r#"
            SELECT
                p.id, p.title, p.description, p.hero_image,
                p.content, p.published_at, p.slug,
                ARRAY[]::TEXT[] as "categories!: Vec<Category>"
            FROM blog_posts p
            WHERE p.slug = $1
            "#,
            slug
        )
        .fetch_one(&self.0)
        .await?;

        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT c.id, c.name, c.slug, c.description
            FROM categories c
            JOIN blog_posts_categories pc ON c.id = pc.category_id
            WHERE pc.blog_post_id = $1
            ORDER BY c.name
            "#,
            post.id
        )
        .fetch_all(&self.0)
        .await?;

        post.categories = categories;
        Ok(post.into_post())
    }
}
