use axum::{routing::get, Router};
use crate::db::DatabaseManager;
use sqlx::PgPool;
use crate::handlers::user::{get_user_by_id, get_users};

pub fn router(pool: PgPool) -> Router<DatabaseManager> {
    Router::new()
        .route("/user/{id}", get(get_user_by_id))
        .route("/user", get(get_users))
        .with_state(pool)
}