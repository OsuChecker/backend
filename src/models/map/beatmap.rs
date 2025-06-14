use serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime, BigDecimal};

#[derive(Debug, Serialize, Deserialize)]
pub struct Beatmap {
    pub id: i32,
    pub beatmapset_id: i32,
    pub version: String,
    pub difficulty_rating: BigDecimal,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub max_combo: i32,
    pub drain_time: i32,
    pub total_time: i32,
    pub bpm: BigDecimal,
    pub cs: BigDecimal,
    pub ar: BigDecimal,
    pub od: BigDecimal,
    pub hp: BigDecimal,
    pub mode: i32,
    pub status: String,
    pub hit_length: i32,
    pub file_md5: String,
    pub file_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Beatmap {
    pub async fn get_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM beatmap WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(record)
    }

    pub async fn get_by_beatmapset(pool: &sqlx::Pool<sqlx::Postgres>, beatmapset_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        let records = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM beatmap WHERE beatmapset_id = $1
            "#,
            beatmapset_id
        )
        .fetch_all(pool)
        .await?;
        
        Ok(records)
    }

    pub async fn get_by_md5(pool: &sqlx::Pool<sqlx::Postgres>, md5: &str) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM beatmap WHERE file_md5 = $1
            "#,
            md5
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(record)
    }
}
