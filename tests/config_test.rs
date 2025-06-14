use template_axum_sqlx_api::config::Config;

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.server.host, "127.0.0.1");
    assert_eq!(config.server.port, 3000);
    assert_eq!(config.database.url, "postgres://postgres:postgres@localhost:5432/template_db");
    assert_eq!(config.database.max_connections, 10);
    assert_eq!(config.database.min_connections, 1);
    assert_eq!(config.logging.level, "info");
    assert_eq!(config.logging.format, "json");
}

#[test]
fn test_config_server_address() {
    let config = Config::default();
    assert_eq!(config.server_address(), "127.0.0.1:3000");
}



#[test]
fn test_config_load(){
    let path = include_str!("./assets/config.toml");
    let config = Config::load(path).unwrap();
    assert_eq!(config.server.host, "127.0.0.1");
    assert_eq!(config.server.port, 7128);
    assert_eq!(config.database.url, "postgres://test:test@test:5432/template_db");
    assert_eq!(config.database.max_connections, 10);
    assert_eq!(config.database.min_connections, 1);
    assert_eq!(config.logging.level, "info");
    assert_eq!(config.logging.format, "text");
}   