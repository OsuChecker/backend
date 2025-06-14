use template_axum_sqlx_api::{
    config::{Config, DatabaseConfig},
    db::DatabaseManager,
};

#[tokio::test]
async fn test_database_connection() {
    let config = Config::default();
    let mut db = DatabaseManager::new();
    
    // Test de la connexion
    let result = db.connect(&config).await;
    assert!(result.is_ok(), "Failed to connect to database: {:?}", result.err());

    // Test que le pool est disponible après la connexion
    let _pool = db.get_pool(); // Si on arrive ici sans panic, c'est OK
}

#[tokio::test]
async fn test_database_error_handling() {
    let mut db = DatabaseManager::new();
    let invalid_config = Config {
        database: DatabaseConfig {
            url: "postgres://invalid:invalid@localhost:5432/invalid".to_string(),
            max_connections: 1,
            min_connections: 1,
        },
        ..Config::default()
    };

    // Test avec une configuration invalide
    let result = db.connect(&invalid_config).await;
    assert!(result.is_err(), "Should fail with invalid database URL");
}

#[test]
#[should_panic(expected = "Database not initialized")]
fn test_get_pool_panic() {
    let db = DatabaseManager::new();
    db.get_pool();
} 