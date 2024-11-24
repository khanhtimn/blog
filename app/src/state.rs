use sqlx::PgPool;
use axum::extract::FromRef;
use leptos::prelude::{LeptosOptions, ServerFnError};
use crate::db;

#[derive(FromRef, Clone, Debug)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
    pub db: db::PostRepository,
}

impl AppState {
    pub async fn try_from_leptos_state(leptos_options: LeptosOptions) -> Result<Self, ServerFnError> {
        // Should find another way to handle this
        let database_url = match std::fs::read_to_string("/run/secrets/database_url") {
            Ok(secret) => secret.trim().to_string(),
            Err(_) => std::env::var("DATABASE_URL").unwrap_or_default()
        };

        std::env::set_var("DATABASE_URL", database_url.clone());

        let pool = PgPool::connect(&database_url).await?;
        let db = db::PostRepository::new(pool.clone());

        Ok(Self { leptos_options, pool, db })
    }
}
