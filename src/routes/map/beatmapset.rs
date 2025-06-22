use axum::{
    routing::get,
    Router,
};
use crate::db::DatabaseManager;
use sqlx::PgPool;
use crate::handlers::map::beatmapset::{get_beatmapsets, get_beatmapset_by_id};

pub fn router(pool: PgPool) -> Router<DatabaseManager> {
    Router::new()
        .route("/beatmapsets", get(get_beatmapsets))
        .route("/beatmapsets/{id}", get(get_beatmapset_by_id))
        .with_state(pool)
}