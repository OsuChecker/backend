use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Duration, Utc};
use bcrypt::{hash, verify, DEFAULT_COST};
use axum::http::{HeaderMap, header::AUTHORIZATION};
use crate::models::user::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user_id
    pub username: String,
    pub exp: usize,
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

pub struct AuthService {
    secret: String,
}

impl AuthService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn generate_token(&self, user: &User, expiry_hours: u64) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(expiry_hours as i64))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }

    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash)
    }

    pub async fn login(
        &self,
        pool: &sqlx::Pool<sqlx::Postgres>,
        login_request: LoginRequest,
    ) -> Result<LoginResponse, AuthError> {
        // Récupérer l'utilisateur par nom d'utilisateur
        let user = User::get_by_username(pool, &login_request.username)
            .await
            .map_err(AuthError::Database)?
            .ok_or(AuthError::InvalidCredentials)?;

        // Vérifier le mot de passe
        if !Self::verify_password(&login_request.password, &user.password_hash)
            .map_err(AuthError::Bcrypt)?
        {
            return Err(AuthError::InvalidCredentials);
        }

        // Générer le token
        let token = self.generate_token(&user, 24).map_err(AuthError::Jwt)?;

        // Extraire les rôles de l'utilisateur
        let roles = user.get_roles();

        Ok(LoginResponse { token, user, roles })
    }

    /// Extraire optionnellement les claims depuis les headers HTTP
    pub fn extract_optional_claims(&self, headers: &HeaderMap) -> Option<Claims> {
        // Extraire le token depuis les headers
        let auth_header = headers.get(AUTHORIZATION)?;
        let auth_str = auth_header.to_str().ok()?;
        
        if !auth_str.starts_with("Bearer ") {
            return None;
        }
        
        let token = auth_str.trim_start_matches("Bearer ");
        
        // Vérifier le token
        self.verify_token(token).ok()
    }
}

#[derive(Debug)]
pub enum AuthError {
    Database(sqlx::Error),
    Jwt(jsonwebtoken::errors::Error),
    Bcrypt(bcrypt::BcryptError),
    InvalidCredentials,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::Database(e) => write!(f, "Database error: {}", e),
            AuthError::Jwt(e) => write!(f, "JWT error: {}", e),
            AuthError::Bcrypt(e) => write!(f, "Bcrypt error: {}", e),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
        }
    }
}

impl std::error::Error for AuthError {} 