use serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime, BigDecimal};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreRating {
    pub id: i32,
    pub score_id: i32,
    pub rating_type_id: i32,
    pub rating_value: BigDecimal,
    pub max_rating: Option<BigDecimal>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatingType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl ScoreRating {
    pub async fn get_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM score_rating WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(record)
    }

    pub async fn get_by_score(pool: &sqlx::Pool<sqlx::Postgres>, score_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        let records = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM score_rating WHERE score_id = $1
            "#,
            score_id
        )
        .fetch_all(pool)
        .await?;
        
        Ok(records)
    }

    pub async fn get_by_score_and_type(
        pool: &sqlx::Pool<sqlx::Postgres>, 
        score_id: i32, 
        rating_type: &str
    ) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            SELECT sr.* FROM score_rating sr
            JOIN rating_type rt ON sr.rating_type_id = rt.id
            WHERE sr.score_id = $1 AND rt.name = $2
            "#,
            score_id,
            rating_type
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(record)
    }
}

impl RatingType {
    pub async fn get_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM rating_type WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(record)
    }

    pub async fn get_by_name(pool: &sqlx::Pool<sqlx::Postgres>, name: &str) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM rating_type WHERE name = $1
            "#,
            name
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(record)
    }

    pub async fn get_all_active(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Self>, sqlx::Error> {
        let records = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM rating_type WHERE is_active = true
            "#
        )
        .fetch_all(pool)
        .await?;
        
        Ok(records)
    }
}
