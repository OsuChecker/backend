use axum::{
    routing::{get, post},
    Router,
};
use axum::middleware;
use sqlx::PgPool;
use crate::{
    handlers::ranked::{
        join_queue,
        leave_queue,
        get_queue_status,
        get_match_status,
        set_ready,
    },
    db::DatabaseManager,
    middleware::auth::auth_middleware,
};

pub fn router(pool: PgPool) -> Router<DatabaseManager> {
    Router::new()
        .route("/ranked/queue/join", post(join_queue))
        .route("/ranked/queue/leave", post(leave_queue))
        .route("/ranked/queue/status", get(get_queue_status))
        .route("/ranked/match/status", get(get_match_status))
        .route("/ranked/match/ready", post(set_ready))
        .layer(middleware::from_fn_with_state(pool.clone(), auth_middleware))
        .with_state(pool)
    } 