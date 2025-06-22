use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,
}

#[derive(Debug, Deserialize)]
pub struct FixturesConfig {
    pub enabled: bool,
    pub reset_database: bool,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: Option<LoggingConfig>,
    pub fixtures: Option<FixturesConfig>,
    pub cors: CorsConfig,
    pub auth: AuthConfig,
    pub osu_api: OsuApiConfig,
}

#[derive(Debug, Deserialize)]
pub struct OsuApiConfig {
    client_id: String,
    client_secret: String,
}

impl Config {
    /// Initialise le système de logging
    fn init_logging(level: &str, _format: &str) {
        let env_filter = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new(level))
            .unwrap_or_else(|_| EnvFilter::new("info"));

        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .init();

        info!("Logging initialized with level: {}", level);
    }

    /// Charge la configuration depuis config.toml
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Charger la configuration depuis le fichier TOML
        let config_content = path;
        let config = toml::from_str::<Config>(config_content)?;
        
        // Initialiser le logging avec la configuration
        Self::init_logging(&config.logging.as_ref().map_or("info", |l| &l.level), &config.logging.as_ref().map_or("json", |l| &l.format));

        info!("Configuration loaded successfully. Server will bind to: {}", config.server_address());
        Ok(config)
    }

    /// Retourne l'adresse complète du serveur
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Récupère l'identifiant client pour l'API osu!
    pub fn osu_client_id(&self) -> &str {
        &self.osu_api.client_id
    }

    /// Récupère le secret client pour l'API osu!
    pub fn osu_client_secret(&self) -> &str {
        &self.osu_api.client_secret
    }

    /// Récupère le secret JWT pour l'authentification
    pub fn jwt_secret(&self) -> &str {
        &self.auth.jwt_secret
    }

    /// Récupère la durée d'expiration des tokens JWT en heures
    pub fn jwt_expiry_hours(&self) -> u64 {
        self.auth.jwt_expiry_hours
    }
}

impl Default for Config {
    fn default() -> Self {
        warn!("Using default configuration as no config.toml was found");
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            database: DatabaseConfig {
                url: "postgres://postgres:postgres@localhost:5432/template_db".to_string(),
                max_connections: 10,
                min_connections: 1,
            },
            logging: None,
            fixtures: None,
            cors: CorsConfig {
                allowed_origins: vec![
                    "http://localhost:3000".to_string(),
                    "http://127.0.0.1:3000".to_string(),
                ],
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                    "OPTIONS".to_string(),
                ],
                allowed_headers: vec![
                    "content-type".to_string(),
                    "authorization".to_string(),
                ],
            },
            auth: AuthConfig {
                jwt_secret: "default-secret-key-change-in-production".to_string(),
                jwt_expiry_hours: 24,
            },
            osu_api: OsuApiConfig {
                client_id: "".to_string(),
                client_secret: "".to_string(),
            },
        }
    }
}
