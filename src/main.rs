//! # Template Axum SQLx API
//!
//! Ce module est le point d'entrée principal de l'application.
//! Il configure et démarre le serveur HTTP avec Axum.
//!
//! ## Fonctionnalités
//! - Configuration depuis config.toml
//! - Initialisation de la base de données
//! - Configuration du logging
//! - Configuration CORS
//! - Gestion des erreurs

mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod fixtures;
mod middleware;
pub mod helpers;
pub mod auth;

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::{info, error};
use fixtures::run_fixtures;
use middleware::logging::setup_middleware;
use models::map::beatmap_queue::BeatmapQueue;
use helpers::osuapi::OsuAPI;
/// Point d'entrée principal de l'application.
///
/// Cette fonction :
/// 1. Charge la configuration depuis config.toml
/// 2. Initialise la base de données
/// 3. Configure les routes et les middlewares
/// 4. Démarre le serveur HTTP
#[tokio::main]
async fn main() {

    // Load configuration from config.toml
    let config = config::Config::load(include_str!("../assets/config.toml")).expect("Failed to load configuration");

    // Initialize database
    let mut db = db::DatabaseManager::new();
    db.connect(&config)
        .await
        .expect("Failed to connect to database");

    // Run fixtures if enabled
    if let Some(fixtures_config) = &config.fixtures {
        if fixtures_config.enabled {
            run_fixtures(db.get_pool(), fixtures_config.reset_database)
                .await
                .expect("Failed to run fixtures");
        }
    }
    
    // Initialize the beatmap queue
    let api = OsuAPI::new(
        config.osu_client_id().to_string(),
        config.osu_client_secret().to_string()
    );
    
    // Test the API connection
    info!("Testing osu! API connection...");
    match api.get_beatmap_by_id("1").await {
        Ok(_) => info!("osu! API connection successful"),
        Err(e) => error!("Failed to connect to osu! API: {}", e),
    }
    
    BeatmapQueue::init(db.get_pool().clone(), api)
        .await
        .expect("Failed to initialize beatmap queue");
    info!("Beatmap queue initialized");

    // Build our application with a route
    let mut app = Router::new()
        .merge(routes::create_router(db));

    // CORS for localhost development - Comment/Uncomment as needed
    // app = app.layer(CorsLayer::permissive()); // Permissive CORS for all origins
     app = app.layer(
         CorsLayer::new()
            .allow_origin("*".parse::<axum::http::HeaderValue>().unwrap())
            .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::PUT, axum::http::Method::DELETE])
            .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION])
    );

    // Apply logging middleware
    let app = setup_middleware(app);

    // Run it
    let addr: SocketAddr = config
        .server_address()
        .parse()
        .expect("Invalid server address");
    info!("listening on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
