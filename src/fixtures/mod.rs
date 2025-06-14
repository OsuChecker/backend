mod score;
mod common;
use sqlx::{Pool, Postgres};
use tracing::{info, warn};
use score::{create_dummy, clean_dummy};

async fn clean_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Cleaning fixtures...");

    clean_dummy(pool).await.map_err(|e| {
        warn!("Error cleaning fixtures: {}", e);
        e
    })
}

async fn load_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Loading fixtures...");

    create_dummy(pool).await.map_err(|e| {
        warn!("Error loading fixtures: {}", e);
        e
    })
}
/// Structure pour gérer les fixtures de test
pub async fn run_fixtures(pool: &Pool<Postgres>, clean : bool) -> Result<(), sqlx::Error> {
    info!("Running fixtures...");

    // delete this, it's just an example of use
    if clean {
        clean_fixtures(pool).await?;
    }
    load_fixtures(pool).await?;
    
    info!("Fixtures run successfully");
    Ok(())
}