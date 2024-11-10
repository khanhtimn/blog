use sqlx::PgPool;
use std::env;
use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;
use crate::db;

#[derive(FromRef, Clone, Debug)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
    pub db: db::PostRepository,
}

impl AppState {
    pub async fn try_from_leptos_state(leptos_options: LeptosOptions) -> anyhow::Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let pool = PgPool::connect(&database_url).await?;
        let db = db::PostRepository::new(pool.clone());

        Ok(Self { leptos_options, pool, db })
    }
}
