use axum::{
    routing::get_service,
    Router,
};
use sqlx::PgPool;
use tower_http::services::ServeDir;
use crate::db::DatabaseManager;

/// Router pour servir des fichiers publics statiques (images, etc.)
pub fn router(_db: PgPool) -> Router<DatabaseManager> {
    Router::new()
        .nest_service(
            "/public", 
            get_service(ServeDir::new("public"))
                .handle_error(|error| async move {
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Erreur lors de l'accès au fichier: {}", error),
                    )
                }),
        )
} 