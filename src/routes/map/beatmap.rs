use axum::{routing::get, Router};
use crate::db::DatabaseManager;
use sqlx::PgPool;
use crate::handlers::map::beatmap::{get_beatmap, get_random};

pub fn router(pool: PgPool) -> Router<DatabaseManager> {
    Router::new()
        .route("/beatmapset/{beatmapset_id}/beatmap", get(get_beatmap))
        .route("/beatmap/random", get(get_random))
        .with_state(pool)
}