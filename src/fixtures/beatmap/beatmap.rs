

use crate::fixtures::common::FixtureManager;
use sqlx::{Pool, Postgres};
use fake::{Fake, Faker};
use tracing::info;
use crate::models::map::beatmap::CreateBeatmap;

pub fn create_beatmap_faker(number: u32) -> Vec<CreateBeatmap> {
    (0..number)
        .map(|_| {
            let mut beatmap = Faker.fake::<CreateBeatmap>();
            beatmap.beatmapset_id = 1;
            beatmap
        })
        .collect()
}

pub async fn create_beatmap_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Creating beatmap...");
    let fixture_manager = FixtureManager::new(pool.clone());
    
    fixture_manager.submit_fixtures(create_beatmap_faker(10), "beatmap").await?;
    Ok(())
}

pub async fn clean_beatmap_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Cleaning beatmap...");
    let fixture_manager = FixtureManager::new(pool.clone());
    fixture_manager.cleanup_fixtures("beatmap").await?;
    Ok(())
}