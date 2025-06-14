//! # Help Routes Module
//!
//! Ce module configure les routes d'aide et de diagnostic de l'API.

use axum::{routing::get, Router};
use crate::{db::DatabaseManager, handlers::help};

/// Créer le routeur pour les routes d'aide
pub fn router() -> Router<DatabaseManager> {
    Router::new()
        .route("/help/health", get(help::health_check))
        .route("/help/health-light", get(help::health_light))
        .route("/help/info", get(help::info))
        .route("/help/ping", get(help::ping))
} 