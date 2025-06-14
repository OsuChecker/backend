use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub country: String,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
    pub is_verified: bool,
    pub last_visit: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub async fn get_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "user" WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
    
        Ok(record)
    }

    pub async fn get_by_username(pool: &sqlx::Pool<sqlx::Postgres>, username: &str) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "user" WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;
    
        Ok(record)
    }
}
