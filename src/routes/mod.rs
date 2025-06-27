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
use axum::{Router, routing::get};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use sqlx::PgPool;

// Re-export all route modules here
pub mod help;
pub mod user;
pub mod map;
pub mod score;
pub mod public;
pub mod auth;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::user::get_user_by_id,
        crate::handlers::user::get_users,
        crate::handlers::map::beatmap::get_beatmap,
        crate::handlers::map::beatmapset::get_beatmapsets,
        crate::handlers::map::beatmapset::get_beatmapset_by_id,
        crate::handlers::score::score::get_leaderboard,
        crate::handlers::score::pp_calculator::calculate_missing_pp,
    ),
    components(
        schemas(
            crate::models::user::user::User,
            crate::models::map::beatmap::BeatmapSchema,
            crate::models::map::beatmapset::BeatmapsetSchema,
            crate::handlers::score::score::LeaderboardParams,
            crate::models::score::score::LeaderboardSchema,
            crate::handlers::score::pp_calculator::PPCalculationParams,
            crate::handlers::score::pp_calculator::PPCalculationResponse,
        )
    ),
    tags(
        (name = "User", description = "User management endpoints"),
        (name = "Beatmap", description = "Beatmap management endpoints"),
        (name = "Beatmapsets", description = "Beatmapset management endpoints"),
        (name = "Score", description = "Score management endpoints"),
    )
)]
pub struct ApiDoc;

pub fn create_router(db: DatabaseManager) -> Router {
    Router::new()
        // Page d'accueil
        .route("/", get(crate::handlers::home::home))
        .nest("/api", help::router())
        .nest("/api", user::router(db.get_pool().clone()))
        .nest("/api", map::beatmap::router(db.get_pool().clone()))
        .nest("/api", map::beatmapset::router(db.get_pool().clone()))
        .nest("/api", score::score::router(db.get_pool().clone()))
        .nest("/api", auth::router(db.get_pool().clone()))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(public::router(db.get_pool().clone()))
        .with_state(db)
}
