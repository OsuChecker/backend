mod score;
mod common;
mod user;
mod beatmap;

use sqlx::{Pool, Postgres};
use tracing::{info, warn};
use score::score::{create_score_fixtures, clean_score_fixtures};
use user::user::{create_user_fixtures, clean_user_fixtures};
use beatmap::beatmapset::{create_beatmapset_fixtures, clean_beatmapset_fixtures};
use beatmap::beatmap::{create_beatmap_fixtures, clean_beatmap_fixtures};

async fn clean_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Cleaning fixtures...");

    clean_user_fixtures(pool).await.map_err(|e| {
        warn!("Error cleaning fixtures: {}", e);
        e
    })?;
    clean_beatmapset_fixtures(pool).await.map_err(|e| {
        warn!("Error cleaning fixtures: {}", e);
        e
    })?;
    clean_beatmap_fixtures(pool).await.map_err(|e| {
        warn!("Error cleaning fixtures: {}", e);
        e
    })?;
    clean_score_fixtures(pool).await.map_err(|e| {
         warn!("Error cleaning fixtures: {}", e);
         e
     })
}

async fn load_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Loading fixtures...");

    create_user_fixtures(pool).await.map_err(|e| {
        warn!("Error loading fixtures: {}", e);
        e
    })?;
    create_beatmapset_fixtures(pool).await.map_err(|e| {
        warn!("Error loading fixtures: {}", e);
        e
    })?;
    create_beatmap_fixtures(pool).await.map_err(|e| {
        warn!("Error loading fixtures: {}", e);
        e
    })?;
    create_score_fixtures(pool).await.map_err(|e| {
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