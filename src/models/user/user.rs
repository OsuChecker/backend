use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use fake::Dummy;
use validator::Validate;
use fake::faker::internet::en::{FreeEmail, Username};
use fake::faker::address::en::CountryCode;
use utoipa::ToSchema;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, Dummy, Validate)]
pub struct CreateUser {
    #[dummy(faker = "3..32")]
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[dummy(faker = "FreeEmail()")]
    #[validate(email)]
    pub email: String,
    #[dummy(faker = "8..128")]
    #[validate(length(min = 8, max = 128))]
    pub password_hash: String,
    #[dummy(faker = "CountryCode()")]
    #[validate(length(min = 2, max = 2))]
    pub country: String,
}

impl User {
    pub async fn get_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE id = $1
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
            SELECT * FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;
    
        Ok(record)
    }

    pub async fn get_all(pool: &sqlx::Pool<sqlx::Postgres>, page: i64, per_page: i64) -> Result<Vec<Self>, sqlx::Error> {
        let offset = (page - 1) * per_page;
        let records = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            ORDER BY id
            LIMIT $1 OFFSET $2
            "#,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(records)
    }
}
