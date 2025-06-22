use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::auth::{AuthService, Claims};

pub async fn auth_middleware(
    State(auth_service): State<AuthService>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extraire le token du header Authorization
    let token = extract_token_from_headers(&headers)?;
    
    // Vérifier le token
    let claims = auth_service
        .verify_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Ajouter les claims à la requête pour les handlers suivants
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}

pub async fn optional_auth_middleware(
    State(auth_service): State<AuthService>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    // Essayer d'extraire le token, mais ne pas échouer si absent
    if let Ok(token) = extract_token_from_headers(&headers) {
        // Si token présent, essayer de le vérifier
        if let Ok(claims) = auth_service.verify_token(&token) {
            // Ajouter les claims si le token est valide
            request.extensions_mut().insert(claims);
        }
    }
    
    // Continuer dans tous les cas (avec ou sans authentification)
    next.run(request).await
}

fn extract_token_from_headers(headers: &HeaderMap) -> Result<String, StatusCode> {
    let auth_header = headers
        .get(AUTHORIZATION)
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(auth_header.trim_start_matches("Bearer ").to_string())
}

// Extension trait pour extraire facilement les claims dans les handlers
pub trait ClaimsExtension {
    fn claims(&self) -> Option<&Claims>;
}

impl ClaimsExtension for Request {
    fn claims(&self) -> Option<&Claims> {
        self.extensions().get::<Claims>()
    }
} 