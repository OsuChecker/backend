use serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime, BigDecimal, JsonValue};
use serde_json;
use fake::{Fake, Faker, Dummy};
use utoipa::ToSchema;
use crate::models::user::user::{SimplfiedUser};
use crate::models::map::beatmap::Beatmap;
#[derive(Debug, Serialize, Deserialize, Dummy, ToSchema, Clone)]
pub struct ScoreStatistics {
    pub count_300: i32,
    pub count_100: i32,
    pub count_50: i32,
    pub count_miss: i32,
    pub count_katu: i32,
    pub count_geki: i32,
}
impl From<JsonValue> for ScoreStatistics {
    fn from(value: JsonValue) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl From<JsonValue> for SimplfiedUser {
    fn from(value: JsonValue) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Dummy)]
pub struct Score {
    pub id: i32,
    pub user_id: i32,
    pub beatmap_id: i32,
    pub score: i32,
    pub max_combo: i32,
    pub perfect: bool,
    pub statistics: ScoreStatistics,
    pub mods: i32,
    pub accuracy: BigDecimal,
    pub rank: String,
    pub replay_available: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Leaderboard{
    pub id: i32,
    pub beatmap_id: i32,
    pub score: i32,
    pub max_combo: i32,
    pub perfect: bool,
    pub statistics: ScoreStatistics,
    pub mods: i32,
    pub accuracy: BigDecimal,
    pub rank: String,
    pub replay_available: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub hash: Option<String>,
    pub player : SimplfiedUser,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LeaderboardSchema {
    pub id: i32,
    pub beatmap_id: i32,
    pub score: i32,
    pub max_combo: i32,
    pub perfect: bool,
    pub statistics: ScoreStatistics,
    pub mods: i32,
    pub accuracy: f64,
    pub rank: String,
    pub replay_available: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub hash: Option<String>,
    pub player: SimplfiedUser,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScoreSchema {
    pub id: i32,
    pub user_id: i32,
    pub beatmap_id: i32,
    pub score: i32,
    pub max_combo: i32,
    pub perfect: bool,
    pub statistics: ScoreStatistics,
    pub mods: i32,
    pub accuracy: f64,
    pub rank: String,
    pub replay_available: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Dummy, Clone)]
pub struct CreateScore {
    pub user_id: i32,
    pub beatmap_hash: String,
    pub score: i32,
    pub max_combo: i32,
    pub perfect: bool,
    pub statistics: ScoreStatistics,
    pub mods: i32,
    pub accuracy: BigDecimal,
    pub rank: String,
    pub replay_available: bool,
    pub hash: Option<String>,
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

    pub async fn get_leaderboard(pool: &sqlx::Pool<sqlx::Postgres>, beatmap_id: i32, mods: Option<i32>, page: i64, per_page: i64) -> Result<Vec<Leaderboard>, sqlx::Error> {
        let offset = (page - 1) * per_page;
        
        let records = match mods {
            Some(mods_value) => {
                sqlx::query_as!(
                    Leaderboard,
                    r#"
                        SELECT 
                            best_scores.id,
                            best_scores.beatmap_id,
                            best_scores.score,
                            best_scores.max_combo,
                            best_scores.perfect,
                            best_scores.statistics AS "statistics!: JsonValue",
                            best_scores.mods,
                            best_scores.accuracy,
                            best_scores.rank,
                            best_scores.replay_available,
                            best_scores.created_at,
                            best_scores.updated_at,
                            best_scores.hash,
                            json_build_object(
                                'id', u.id,
                                'username', u.username,
                                'country', u.country
                            ) as "player!: JsonValue"
                        FROM (
                            SELECT DISTINCT ON (user_id)
                                id,
                                user_id,
                                beatmap_id,
                                score,
                                max_combo,
                                perfect,
                                statistics,
                                mods,
                                accuracy,
                                rank,
                                replay_available,
                                created_at,
                                updated_at,
                                hash
                            FROM score
                            WHERE beatmap_id = $1 AND mods = $2
                            ORDER BY user_id, score DESC
                        ) AS best_scores
                        JOIN users u ON best_scores.user_id = u.id
                        ORDER BY best_scores.score DESC
                        LIMIT $3 OFFSET $4

                    "#,
                    beatmap_id,
                    mods_value,
                    per_page,
                    offset
                )
                .fetch_all(pool)
                .await?
            },
            None => {
                sqlx::query_as!(
                    Leaderboard,
                    r#"
                        SELECT 
                            best_scores.id,
                            best_scores.beatmap_id,
                            best_scores.score,
                            best_scores.max_combo,
                            best_scores.perfect,
                            best_scores.statistics AS "statistics!: JsonValue",
                            best_scores.mods,
                            best_scores.accuracy,
                            best_scores.rank,
                            best_scores.replay_available,
                            best_scores.created_at,
                            best_scores.updated_at,
                            best_scores.hash,
                            json_build_object(
                                'id', u.id,
                                'username', u.username,
                                'country', u.country
                            ) as "player!: JsonValue"
                        FROM (
                            SELECT DISTINCT ON (user_id)
                                id,
                                user_id,
                                beatmap_id,
                                score,
                                max_combo,
                                perfect,
                                statistics,
                                mods,
                                accuracy,
                                rank,
                                replay_available,
                                created_at,
                                updated_at,
                                hash
                            FROM score
                            WHERE beatmap_id = $1 
                            ORDER BY user_id, score DESC
                        ) AS best_scores
                        JOIN users u ON best_scores.user_id = u.id
                        ORDER BY best_scores.score DESC
                        LIMIT $2 OFFSET $3
                    "#,
                    beatmap_id,
                    per_page,
                    offset
                )
                .fetch_all(pool)
                .await?
            }
        };
        
        Ok(records)
    }

    pub async fn create(pool: &sqlx::Pool<sqlx::Postgres>, create_score: CreateScore) -> Result<Self, sqlx::Error> {
        let statistics_json = serde_json::to_value(create_score.statistics)
            .map_err(|e| sqlx::Error::Protocol(format!("Failed to serialize statistics: {}", e)))?;
        let get_beatmap = Beatmap::get_by_md5(pool, &create_score.beatmap_hash).await?.unwrap();
        let record: Score = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO score (
                user_id, beatmap_id, score, max_combo, perfect,
                statistics, mods, accuracy, rank, replay_available, hash
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
            create_score.user_id,
            get_beatmap.id,
            create_score.score,
            create_score.max_combo,
            create_score.perfect,
            statistics_json as _,
            create_score.mods,
            create_score.accuracy as _,
            create_score.rank,
            create_score.replay_available,
            create_score.hash
        )
        .fetch_one(pool)
        .await?;

        Ok(record)
    }
}
