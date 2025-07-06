use axum::{
    body::Body,
    response::IntoResponse,
    extract::{Request, Json, State},
    http,
    http::{Response, StatusCode},
    middleware::Next,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use crate::models::user::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String, // user_id
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub country: String,
}

pub struct AuthError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}

// Réutilisation des fonctions de hash de l'ancien code
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn encode_jwt(user: &User) -> Result<String, StatusCode> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

    let now = Utc::now();
    let expire = Duration::hours(24);
    let exp = (now + expire).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        iat,
        exp,
        sub: user.id.to_string(),
        username: user.username.clone(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, StatusCode> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

    decode(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn authorize(
    State(pool): State<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Invalid header format".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?,
        None => {
            return Err(AuthError {
                message: "Missing authorization header".to_string(),
                status_code: StatusCode::FORBIDDEN,
            })
        }
    };

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AuthError {
            message: "Invalid token format".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    let token_data = match decode_jwt(token) {
        Ok(data) => data,
        Err(_) => {
            return Err(AuthError {
                message: "Invalid token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            })
        }
    };

    // Récupérer l'utilisateur depuis la base de données
    let user = match User::get_by_id(&pool, token_data.claims.sub.parse().unwrap()).await {
        Ok(Some(user)) => user,
        _ => {
            return Err(AuthError {
                message: "User not found".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            })
        }
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(login_data): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Récupérer l'utilisateur
    let user = match User::get_by_username(&pool, &login_data.username).await {
        Ok(Some(user)) => user,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // Vérifier le mot de passe
    if !verify_password(&login_data.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Générer le JWT
    let token = encode_jwt(&user).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extraire les rôles
    let roles = user.get_roles();

    Ok(Json(LoginResponse { token, user, roles }))
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(register_data): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Hash du mot de passe
    let password_hash = hash_password(&register_data.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Créer l'utilisateur
    let user = User::create(
        &pool,
        crate::models::user::user::CreateUser {
            username: register_data.username,
            email: register_data.email,
            password: password_hash,
            country: register_data.country,
        },
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Générer le token
    let token = encode_jwt(&user)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extraire les rôles
    let roles = user.get_roles();

    Ok(Json(LoginResponse { token, user, roles }))
} 

