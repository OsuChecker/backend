use template_axum_sqlx_api::{
    config::Config,
    db::DatabaseManager,
    fixtures,
};
use sqlx::Row;
use sqlx::{Pool, Postgres};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

async fn get_count(pool: &Pool<Postgres>) -> i64 {
    sqlx::query("SELECT COUNT(*) FROM dummy")
        .fetch_one(pool)
        .await
        .expect("Failed to query dummy table")
        .get(0)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_run_fixtures() {
    let _lock = TEST_MUTEX.lock().unwrap();
    // Setup database connection
    let config = Config::default();
    let mut db = DatabaseManager::new();
    db.connect(&config).await.expect("Failed to connect to database");
    let pool = db.get_pool();

    // Test running fixtures with clean=true
    let result = fixtures::run_fixtures(pool, true).await;
    assert!(result.is_ok(), "Failed to run fixtures: {:?}", result.err());

    // Verify that dummy table has data
    let count: i64 = get_count(pool).await;
    assert!(count == 100, "Dummy table should contain 100 data after fixtures but got {}", count);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_run_fixtures_without_clean() {
    let _lock = TEST_MUTEX.lock().unwrap();
    // Setup database connection
    let config = Config::default();
    let mut db = DatabaseManager::new();
    db.connect(&config).await.expect("Failed to connect to database");
    let pool = db.get_pool();

    // Test running fixtures with clean=false
    let result = fixtures::run_fixtures(pool, true).await;
    assert!(result.is_ok(), "Failed to run fixtures with cleaning: {:?}", result.err());
    let first_count: i64 = get_count(pool).await;

    let result = fixtures::run_fixtures(pool, false).await;
    assert!(result.is_ok(), "Failed to run fixtures without cleaning: {:?}", result.err());

    // Verify that dummy table has data
    let second_count: i64 = get_count(pool).await;
    assert!(second_count == first_count*2, "Dummy table should contain more data after fixtures but got {} and {}", first_count, second_count);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_fixtures_cleanup() {
    let _lock = TEST_MUTEX.lock().unwrap();
    // Setup database connection
    let config = Config::default();
    let mut db = DatabaseManager::new();
    db.connect(&config).await.expect("Failed to connect to database");
    let pool = db.get_pool();

    // First run fixtures
    fixtures::run_fixtures(pool, true).await.expect("Failed to run fixtures");

    // Verify first run
    let first_count: i64 = get_count(pool).await;
    // Then run fixtures again with clean=true to test cleanup
    let result = fixtures::run_fixtures(pool, true).await;
    assert!(result.is_ok(), "Failed to clean and rerun fixtures: {:?}", result.err());

    // Verify second run
    let second_count: i64 = get_count(pool).await;
    assert!(second_count == first_count, "Dummy table should have 100 data loading fixtures but got {}", second_count);
} 