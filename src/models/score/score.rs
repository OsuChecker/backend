use serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime, BigDecimal, JsonValue};

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    pub id: i32,
    pub user_id: i32,
    pub beatmap_id: i32,
    pub score: i32,
    pub max_combo: i32,
    pub perfect: bool,
    pub statistics: JsonValue,
    pub mods: i32,
    pub accuracy: BigDecimal,
    pub rank: String,
    pub replay_available: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Score {
    pub async fn get_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM score WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(record)
    }

    pub async fn get_by_user(pool: &sqlx::Pool<sqlx::Postgres>, user_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        let records = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM score WHERE user_id = $1 ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;
        
        Ok(records)
    }

    pub async fn get_by_beatmap(pool: &sqlx::Pool<sqlx::Postgres>, beatmap_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        let records = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM score WHERE beatmap_id = $1 ORDER BY score DESC
            "#,
            beatmap_id
        )
        .fetch_all(pool)
        .await?;
        
        Ok(records)
    }

    pub async fn get_user_best(pool: &sqlx::Pool<sqlx::Postgres>, user_id: i32, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        let records = sqlx::query_as!(
            Self,
            r#"
            SELECT s.* FROM score s
            JOIN score_rating sr ON s.id = sr.score_id
            JOIN rating_type rt ON sr.rating_type_id = rt.id
            WHERE s.user_id = $1 AND rt.name = 'pp'
            ORDER BY sr.rating_value DESC
            LIMIT $2
            "#,
            user_id,
            limit
        )
        .fetch_all(pool)
        .await?;
        
        Ok(records)
    }

    pub async fn get_user_best_on_beatmap(pool: &sqlx::Pool<sqlx::Postgres>, user_id: i32, beatmap_id: i32) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM score 
            WHERE user_id = $1 AND beatmap_id = $2 
            ORDER BY score DESC 
            LIMIT 1
            "#,
            user_id,
            beatmap_id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(record)
    }
}
