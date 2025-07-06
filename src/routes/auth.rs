use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use crate::middleware::auth::auth_middleware;
use sqlx::PgPool;
use crate::db::DatabaseManager;
use crate::handlers::auth::me;
use crate::auth::{login, register};

pub fn router(pool: PgPool) -> Router<DatabaseManager> {
    let public_routes = Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register));

    let protected_routes = Router::new()
        .route("/auth/me", get(me))
        .layer(middleware::from_fn_with_state(pool.clone(), auth_middleware));

    public_routes
        .merge(protected_routes)
        .with_state(pool)
} 