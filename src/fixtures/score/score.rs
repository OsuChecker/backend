use crate::fixtures::common::FixtureManager;
use sqlx::{Pool, Postgres};
use fake::{Fake, Faker};
use tracing::info;
use crate::models::score::score::CreateScore;
use crate::models::score::score::ScoreStatistics;
use sqlx::types::BigDecimal;
use std::str::FromStr;

pub fn create_score_faker(number: u32) -> Vec<CreateScore> {
    (0..number)
        .map(|_| Faker.fake::<CreateScore>())
        .collect()
}

pub fn create_score_manual(user_id : i32, beatmap_id : i32, score : i32, max_combo : i32,
                     perfect : bool, statistics : ScoreStatistics, mods : i32, 
                     accuracy : BigDecimal, rank : String, replay_available : bool) -> CreateScore {
    CreateScore {
            user_id: user_id,
            beatmap_id: beatmap_id,
            score: score,
            max_combo: max_combo,
            perfect: perfect,
            statistics: statistics,
            mods: mods,
            accuracy: accuracy,
            rank: rank,
            replay_available: replay_available,
    }
}




pub async fn create_score_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Creating score...");
    let fixture_manager = FixtureManager::new(pool.clone());
    let score_stats = ScoreStatistics {count_300: 100, count_100: 0, count_50: 0, count_miss: 0, count_katu: 0, count_geki: 0};

    let score = create_score_manual(1, 1, 1000, 100, true, score_stats, 0, BigDecimal::from_str("100.0").unwrap(), "A".to_string(), true);

    fixture_manager.submit_fixtures(vec![score], "score").await?;
    Ok(())
}

pub async fn clean_score_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Cleaning score...");
    let fixture_manager = FixtureManager::new(pool.clone());
    fixture_manager.cleanup_fixtures("score").await?;
    Ok(())
}