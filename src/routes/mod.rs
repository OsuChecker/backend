//! # Routes Module
//!
//! Ce module gère la configuration des routes de l'API.
//! Il permet d'organiser les routes par domaine fonctionnel et de les combiner
//! dans un routeur Axum unique.
//!
//! ## Utilisation
//!
//! Pour ajouter de nouvelles routes :
//! 1. Créez un nouveau module dans le dossier `routes/`
//! 2. Implémentez une fonction `router()` qui retourne un `Router`
//! 3. Ajoutez le module dans ce fichier
//! 4. Utilisez `merge()` pour combiner les routes

use crate::db::DatabaseManager;
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

// Re-export all route modules here
pub mod help;

#[derive(OpenApi)]
#[openapi(paths(crate::handlers::help::health_check, crate::handlers::help::health_light,
                crate::handlers::help::info, crate::handlers::help::ping))]
struct ApiDoc;

pub fn create_router(db: DatabaseManager) -> Router {
    Router::new()
        .nest("/api", help::router())
        .merge(SwaggerUi::new("/api/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()))
        // Add your other route modules here
        // Example:
        // .nest("/api", user::router())
        // .nest("/api", product::router())
        .with_state(db)
}
