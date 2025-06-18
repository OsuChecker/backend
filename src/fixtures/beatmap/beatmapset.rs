use crate::fixtures::common::FixtureManager;
use sqlx::{Pool, Postgres};
use fake::{Fake, Faker};
use tracing::info;
use crate::models::map::beatmapset::CreateBeatmapset;
use fake::faker::lorem::en::Words;

pub fn create_beatmapset_faker(number: u32) -> Vec<CreateBeatmapset> {
    (0..number)
        .map(|_| {
            let mut beatmapset = Faker.fake::<CreateBeatmapset>();
            beatmapset.creator_id = 1;
            beatmapset.tags = Some(Words(3..8).fake::<Vec<String>>().into_iter().collect());
            beatmapset
        })
        .collect()
}

pub async fn create_beatmapset_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Creating beatmapset...");
    let fixture_manager = FixtureManager::new(pool.clone());
    
    fixture_manager.submit_fixtures(create_beatmapset_faker(10), "beatmapset").await?;
    Ok(())
}

pub async fn clean_beatmapset_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Cleaning beatmapset...");
    let fixture_manager = FixtureManager::new(pool.clone());
    fixture_manager.cleanup_fixtures("beatmapset").await?;
    Ok(())
}