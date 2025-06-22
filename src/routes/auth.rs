use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use crate::db::DatabaseManager;
use crate::handlers::auth::{login, register, me};

pub fn router(pool: PgPool) -> Router<DatabaseManager> {
    let public_routes = Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register));

    let protected_routes = Router::new()
        .route("/auth/me", get(me));

    public_routes
        .merge(protected_routes)
        .with_state(pool)
} 