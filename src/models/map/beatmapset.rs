use serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime, BigDecimal};

#[derive(Debug, Serialize, Deserialize)]
pub struct Beatmapset {
    pub id: i32,
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub title: String,
    pub title_unicode: Option<String>,
    pub creator_id: i32,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub genre_id: Option<i32>,
    pub language_id: Option<i32>,
    pub status: String,
    pub has_video: bool,
    pub has_storyboard: bool,
    pub is_explicit: bool,
    pub is_featured: bool,
    pub bpm: Option<BigDecimal>,
    pub cover_url: Option<String>,
    pub preview_url: Option<String>,
    pub osu_file_url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Beatmapset {
    pub async fn get_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM beatmapset WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(record)
    }

    pub async fn get_by_creator(pool: &sqlx::Pool<sqlx::Postgres>, creator_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        let records = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM beatmapset WHERE creator_id = $1
            "#,
            creator_id
        )
        .fetch_all(pool)
        .await?;
        
        Ok(records)
    }
}
