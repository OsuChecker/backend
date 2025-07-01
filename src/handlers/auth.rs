use axum::{
    extract::{Request, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use crate::auth::{AuthService, LoginRequest, LoginResponse, AuthError};
use crate::models::user::user::{CreateUser, User};
use crate::db::DatabaseManager;
use sqlx::PgPool;

pub async fn login(
    State(pool): State<PgPool>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let auth_service = AuthService::new(
        std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string())
    );
    
    match auth_service.login(&pool, login_request).await {
        Ok(response) => Ok(Json(response)),
        Err(AuthError::InvalidCredentials) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(create_user): Json<CreateUser>,
) -> Result<Json<Value>, StatusCode> {
    let auth_service = AuthService::new(
        std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string())
    );
    
    // Hasher le mot de passe
    let password_hash = AuthService::hash_password(&create_user.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Créer la structure avec le mot de passe hashé
    let user_with_hash = CreateUser {
        username: create_user.username,
        email: create_user.email,
        password: password_hash, // Maintenant on met le hash dans le champ password
        country: create_user.country,
    };

    // Créer l'utilisateur
    match User::create(&pool, user_with_hash).await {
        Ok(user) => {
            // Générer un token pour l'utilisateur nouvellement créé
            let token = auth_service
                .generate_token(&user, 24)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Json(json!({
                "message": "User created successfully",
                "roles": user.get_roles(),
                "token": token,
                "user": user
            })))
        }
        Err(_) => Err(StatusCode::CONFLICT), // Utilisateur existe déjà
    }
}

pub async fn me(request: Request) -> Result<Json<Value>, StatusCode> {
    let claims = request
        .extensions()
        .get::<crate::auth::Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(Json(json!({
        "user_id": claims.sub,
        "username": claims.username,
        "exp": claims.exp
    })))
} 