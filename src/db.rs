//! # Database Module
//!
//! Ce module gère la connexion et les opérations avec la base de données PostgreSQL.
//! Il utilise SQLx pour les requêtes asynchrones et la gestion du pool de connexions.
//!

use crate::config::Config;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Gestionnaire de base de données.
///
/// Cette structure gère la connexion à la base de données PostgreSQL
/// et fournit un pool de connexions pour les requêtes.
#[derive(Clone)]
pub struct DatabaseManager {
    /// Pool de connexions à la base de données
    pool: Option<PgPool>,
}

impl Default for DatabaseManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseManager {
    /// Crée une nouvelle instance de DatabaseManager.
    ///
    /// # Returns
    ///
    /// * `DatabaseManager` - Une nouvelle instance non connectée
    pub fn new() -> Self {
        Self { pool: None }
    }

    /// Établit la connexion à la base de données.
    ///
    /// Cette méthode :
    /// 1. Utilise la configuration de l'application
    /// 2. Crée un pool de connexions
    /// 3. Stocke le pool dans l'instance
    ///
    /// # Arguments
    ///
    /// * `config` - La configuration de la base de données
    ///
    /// # Returns
    ///
    /// * `Result<(), sqlx::Error>` - Succès ou erreur de connexion
    pub async fn connect(&mut self, config: &Config) -> Result<(), sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .min_connections(config.database.min_connections)
            .connect(&config.database.url)
            .await?;

        self.pool = Some(pool);
        tracing::info!("Connected to database with {} max connections", config.database.max_connections);
        Ok(())
    }

    /// Récupère le pool de connexions.
    ///
    /// # Returns
    ///
    /// * `&PgPool` - Référence au pool de connexions
    ///
    /// # Panics
    ///
    /// Cette méthode panique si la base de données n'a pas été initialisée.
    pub fn get_pool(&self) -> &PgPool {
        self.pool.as_ref().expect("Database not initialized")
    }
}
