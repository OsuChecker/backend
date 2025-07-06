use axum::{routing::{get, post}, Router, middleware};
use crate::db::DatabaseManager;
use sqlx::PgPool;
use crate::handlers::score::score::get_leaderboard;
use crate::handlers::score::loadingscore::load_scores_db;
use crate::handlers::score::pp_calculator::calculate_missing_pp;
use crate::middleware::auth::auth_middleware;

pub fn router(pool: PgPool) -> Router<DatabaseManager> {
    Router::new()
        .route("/leaderboard/{beatmap_id}", get(get_leaderboard))
        .route("/scores/load", post(load_scores_db))
            .layer(middleware::from_fn_with_state(pool.clone(), auth_middleware))
        .route("/scores/calculate-pp", post(calculate_missing_pp))
        .with_state(pool)
}