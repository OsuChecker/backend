use axum::{routing::get, Router};
use crate::db::DatabaseManager;
use sqlx::PgPool;
use crate::handlers::map::beatmap::get_beatmap;

pub fn router(pool: PgPool) -> Router<DatabaseManager> {
    Router::new()
        .route("/beatmapset/{beatmapset_id}/beatmap", get(get_beatmap))
        .with_state(pool)
}