use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use template_axum_sqlx_api::{
    config::Config,
    db::DatabaseManager,
    routes::create_router,
};
use axum::body::to_bytes;

#[tokio::test]
async fn test_health_check() {
    let mut db = DatabaseManager::new();
    db.connect(&Config::default()).await.expect("Failed to connect to test database");
    let app = create_router(db);

    let response = Request::builder()
        .uri("/api/help/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(response).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let health: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(health["status"], "healthy");
    assert!(health["database"]["connected"].as_bool().unwrap());
    assert!(health["system"]["cpu_count"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn test_health_light() {
    let mut db = DatabaseManager::new();
    db.connect(&Config::default()).await.expect("Failed to connect to test database");
    let app = create_router(db);

    let response = Request::builder()
        .uri("/api/help/health-light")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(response).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let health: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(health["status"], "healthy");
    assert!(health["database"]["connected"].as_bool().unwrap());
}

#[tokio::test]
async fn test_info() {
    let mut db = DatabaseManager::new();
    db.connect(&Config::default()).await.expect("Failed to connect to test database");
    let app = create_router(db);

    let response = Request::builder()
        .uri("/api/help/info")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(response).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let info: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(info["name"], "template-axum-sqlx-api");
    assert!(!info["version"].as_str().unwrap().is_empty());
    assert!(!info["description"].as_str().unwrap().is_empty());
    assert!(info["endpoints"].is_array());
}

#[tokio::test]
async fn test_ping() {
    let mut db = DatabaseManager::new();
    db.connect(&Config::default()).await.expect("Failed to connect to test database");
    let app = create_router(db);

    let response = Request::builder()
        .uri("/api/help/ping")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(response).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "pong");
} 