use crate::fixtures::common::FixtureManager;
use sqlx::{Pool, Postgres};
use fake::{Fake, Faker};
use tracing::info;
use crate::models::user::user::CreateUser;

pub fn create_user_faker(number: u32) -> Vec<CreateUser> {
    (0..number)
        .map(|_| Faker.fake::<CreateUser>())
        .collect()
}

pub async fn create_user_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Creating user...");
    let fixture_manager = FixtureManager::new(pool.clone());
    
    fixture_manager.submit_fixtures(create_user_faker(100), "users").await?;
    Ok(())
}

pub async fn clean_user_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Cleaning user...");
    let fixture_manager = FixtureManager::new(pool.clone());
    fixture_manager.cleanup_fixtures("users").await?;
    Ok(())
}