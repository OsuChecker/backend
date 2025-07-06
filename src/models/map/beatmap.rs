use serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime, BigDecimal};
use fake::Dummy;
use utoipa::ToSchema;
use sqlx::PgPool;
use anyhow::Result;
use crate::helpers::osuapi::{OsuAPI, BeatmapResponse};
use crate::models::map::beatmap_queue::BeatmapQueue;

#[derive(Debug, Serialize, Deserialize, Dummy)]
pub struct CreateBeatmap {
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
}

// Struct pour la documentation API (sans BigDecimal)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BeatmapSchema {
    pub id: i32,
    pub beatmapset_id: i32,
    pub version: String,
    pub difficulty_rating: f64,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub max_combo: i32,
    pub drain_time: i32,
    pub total_time: i32,
    pub bpm: f64,
    pub cs: f64,
    pub ar: f64,
    pub od: f64,
    pub hp: f64,
    pub mode: i32,
    pub status: String,
    pub hit_length: i32,
    pub file_md5: String,
    pub file_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    // Convertir en BeatmapSchema pour la documentation API
    pub fn to_schema(&self) -> BeatmapSchema {
        BeatmapSchema {
            id: self.id,
            beatmapset_id: self.beatmapset_id,
            version: self.version.clone(),
            difficulty_rating: self.difficulty_rating.to_string().parse::<f64>().unwrap_or(0.0),
            count_circles: self.count_circles,
            count_sliders: self.count_sliders,
            count_spinners: self.count_spinners,
            max_combo: self.max_combo,
            drain_time: self.drain_time,
            total_time: self.total_time,
            bpm: self.bpm.to_string().parse::<f64>().unwrap_or(0.0),
            cs: self.cs.to_string().parse::<f64>().unwrap_or(0.0),
            ar: self.ar.to_string().parse::<f64>().unwrap_or(0.0),
            od: self.od.to_string().parse::<f64>().unwrap_or(0.0),
            hp: self.hp.to_string().parse::<f64>().unwrap_or(0.0),
            mode: self.mode,
            status: self.status.clone(),
            hit_length: self.hit_length,
            file_md5: self.file_md5.clone(),
            file_path: self.file_path.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    // Get a random beatmap from the database
    pub async fn get_random_beatmap(pool: &sqlx::Pool<sqlx::Postgres>, mode: i32, status: &str, limit_difficulty: Option<BigDecimal>) -> Result<Option<Self>, sqlx::Error> {

        let limit_difficulty = limit_difficulty.unwrap_or_else(|| BigDecimal::from(0)); 

        let record = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM beatmap WHERE mode = $1 AND status = $2 AND difficulty_rating <= $3 ORDER BY RANDOM() LIMIT 1
            "#,
            mode,
            status,
            limit_difficulty
        )
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

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

    pub async fn create(pool: &sqlx::Pool<sqlx::Postgres>, create_beatmap: CreateBeatmap) -> Result<Self, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO beatmap (
                beatmapset_id, version, difficulty_rating, count_circles,
                count_sliders, count_spinners, max_combo, drain_time,
                total_time, bpm, cs, ar, od, hp, mode,
                status, hit_length, file_md5, file_path
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            RETURNING *
            "#,
            create_beatmap.beatmapset_id,
            create_beatmap.version,
            create_beatmap.difficulty_rating as _,
            create_beatmap.count_circles,
            create_beatmap.count_sliders,
            create_beatmap.count_spinners,
            create_beatmap.max_combo,
            create_beatmap.drain_time,
            create_beatmap.total_time,
            create_beatmap.bpm as _,
            create_beatmap.cs as _,
            create_beatmap.ar as _,
            create_beatmap.od as _,
            create_beatmap.hp as _,
            create_beatmap.mode,
            create_beatmap.status,
            create_beatmap.hit_length,
            create_beatmap.file_md5,
            create_beatmap.file_path
        )
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    /// Récupère une beatmap par son hash MD5
    pub async fn get_beatmap_by_hash(pool: &PgPool, hash: &str) -> Result<Option<Self>, sqlx::Error> {
        // Simplement vérifier si la beatmap existe dans notre base
        Self::get_by_md5(pool, hash).await
    }
    
    /// Convertit une réponse de l'API en CreateBeatmap
    fn from_api_response(response: BeatmapResponse) -> CreateBeatmap {
        CreateBeatmap {
            beatmapset_id: response.beatmapset_id,
            version: response.version,
            difficulty_rating: BigDecimal::try_from(response.difficulty_rating).unwrap_or_default(),
            count_circles: 0, // À compléter avec les données de l'API si disponibles
            count_sliders: 0, // À compléter avec les données de l'API si disponibles
            count_spinners: 0, // À compléter avec les données de l'API si disponibles
            max_combo: response.max_combo.unwrap_or(0),
            drain_time: response.drain as i32,
            total_time: response.total_length,
            bpm: BigDecimal::try_from(response.bpm).unwrap_or_default(),
            cs: BigDecimal::try_from(response.cs).unwrap_or_default(),
            ar: BigDecimal::try_from(response.ar).unwrap_or_default(),
            od: BigDecimal::try_from(response.accuracy).unwrap_or_default(),
            hp: BigDecimal::try_from(response.drain).unwrap_or_default(),
            mode: response.mode_int,
            status: response.status,
            hit_length: response.hit_length,
            file_md5: response.checksum.unwrap_or_default(),
            file_path: format!("beatmaps/{}.osu", response.id), // Chemin par défaut
        }
    }
}
