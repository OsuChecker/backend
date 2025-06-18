use axum::{routing::get, Router};
use crate::db::DatabaseManager;
use sqlx::PgPool;
use crate::handlers::map::beatmapset::get_beatmapset;

pub fn router(pool: PgPool) -> Router<DatabaseManager> {
    Router::new()
        .route("/beatmapset", get(get_beatmapset))
        .with_state(pool)
}