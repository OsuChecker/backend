use serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime};
use fake::Dummy;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Dummy)]
pub struct CreateBeatmapset {
    pub osu_id: Option<i32>,
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub title: String,
    pub title_unicode: Option<String>,
    pub creator_id: Option<i32>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: String,
    pub has_video: bool,
    pub has_storyboard: bool,
    pub is_explicit: bool,
    pub is_featured: bool,
    pub cover_url: Option<String>,
    pub preview_url: Option<String>,
    pub osu_file_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BeatmapsetSchema {
    pub id: i32,
    pub artist: String,
    pub osu_id: Option<i32>,
    pub artist_unicode: Option<String>,
    pub title: String,
    pub title_unicode: Option<String>,
    pub creator_id: Option<i32>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: String,
    pub has_video: bool,
    pub has_storyboard: bool,
    pub is_explicit: bool,
    pub is_featured: bool,
    pub cover_url: Option<String>,
    pub preview_url: Option<String>,
    pub osu_file_url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Beatmapset {
    pub id: i32,
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub title: String,
    pub title_unicode: Option<String>,
    pub creator_id: Option<i32>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: String,
    pub has_video: bool,
    pub has_storyboard: bool,
    pub is_explicit: bool,
    pub is_featured: bool,
    pub cover_url: Option<String>,
    pub preview_url: Option<String>,
    pub osu_file_url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub osu_id : Option<i32>,
}

impl Beatmapset {
    pub async fn get_all(pool: &sqlx::Pool<sqlx::Postgres>, page: i64, per_page: i64) -> Result<Vec<Self>, sqlx::Error> {
        let offset = (page - 1) * per_page;
        let records = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM beatmapset
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

    pub async fn get_by_osu_id(pool: &sqlx::Pool<sqlx::Postgres>, osu_id: i32) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM beatmapset WHERE osu_id = $1
            "#,
            osu_id
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

    pub fn to_schema(&self) -> BeatmapsetSchema {
        BeatmapsetSchema {
            id: self.id,
            osu_id: self.osu_id,
            artist: self.artist.clone(),
            artist_unicode: self.artist_unicode.clone(),
            title: self.title.clone(),
            title_unicode: self.title_unicode.clone(),
            creator_id: self.creator_id,
            source: self.source.clone(),
            tags: self.tags.clone(),
            status: self.status.clone(),
            has_video: self.has_video,
            has_storyboard: self.has_storyboard,
            is_explicit: self.is_explicit,
            is_featured: self.is_featured,
            cover_url: self.cover_url.clone(),
            preview_url: self.preview_url.clone(),
            osu_file_url: self.osu_file_url.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub async fn create(pool: &sqlx::Pool<sqlx::Postgres>, create_beatmapset: CreateBeatmapset) -> Result<Self, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO beatmapset (
                artist, artist_unicode, title, title_unicode, creator_id,
                source, tags, status, has_video, has_storyboard,
                is_explicit, is_featured, cover_url, preview_url, osu_file_url, osu_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *
            "#,
            create_beatmapset.artist,
            create_beatmapset.artist_unicode,
            create_beatmapset.title,
            create_beatmapset.title_unicode,
            create_beatmapset.creator_id,
            create_beatmapset.source,
            create_beatmapset.tags as _,
            create_beatmapset.status,
            create_beatmapset.has_video,
            create_beatmapset.has_storyboard,
            create_beatmapset.is_explicit,
            create_beatmapset.is_featured,
            create_beatmapset.cover_url,
            create_beatmapset.preview_url,
            create_beatmapset.osu_file_url,
            create_beatmapset.osu_id
        )
        .fetch_one(pool)
        .await?;

        Ok(record)
    }
}
