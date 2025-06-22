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
            beatmapset.creator_id = Some(1);
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

pub fn generate_beatmapset() -> CreateBeatmapset {
    let mut beatmapset = CreateBeatmapset {
        osu_id: Some(Faker.fake::<i32>()),
        artist: Faker.fake::<String>(),
        artist_unicode: Some(Faker.fake::<String>()),
        title: Faker.fake::<String>(),
        title_unicode: Some(Faker.fake::<String>()),
        creator_id: None,
        source: Some(Faker.fake::<String>()),
        tags: Some(vec![Faker.fake::<String>(), Faker.fake::<String>()]),
        status: "ranked".to_string(),
        has_video: Faker.fake::<bool>(),
        has_storyboard: Faker.fake::<bool>(),
        is_explicit: Faker.fake::<bool>(),
        is_featured: Faker.fake::<bool>(),
        cover_url: Some(Faker.fake::<String>()),
        preview_url: Some(Faker.fake::<String>()),
        osu_file_url: Some(Faker.fake::<String>()),
    };
    
    // Pour les tests, on peut définir creator_id comme Some(1)
    beatmapset.creator_id = Some(1);
    
    beatmapset
}