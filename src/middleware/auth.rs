use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use crate::auth;
use crate::models::user::user::User;

pub async fn auth_middleware(
    State(pool): State<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(axum::http::header::AUTHORIZATION)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_header = auth_header
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token_data = auth::decode_jwt(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Récupérer l'utilisateur depuis la base de données
    let user = match User::get_by_id(&pool, token_data.claims.sub.parse().unwrap()).await {
        Ok(Some(user)) => user,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
} 