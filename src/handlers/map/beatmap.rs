use crate::models::map::beatmap::{Beatmap, BeatmapSchema};
use axum::extract::{State, Path, Query};
use axum::{response::Json, http::StatusCode};
use sqlx::PgPool;
use serde::Deserialize;
use bigdecimal::BigDecimal;
use crate::models::map::beatmap::RandomBeatmapQuerySchema;
use tracing::info;

#[utoipa::path(
    get,
    path = "/api/beatmapset/{beatmapset_id}/beatmap",
    tag = "Beatmap",
    responses(
        (status = 200, description = "Beatmaps found", body = Vec<BeatmapSchema>),
        (status = 404, description = "Beatmapset not found")
    ),
    summary = "Get beatmaps by beatmapset id",
    description = "Get all beatmaps by beatmapset id"
)]
pub async fn get_beatmap(
    State(pool): State<PgPool>,
    Path(beatmapset_id): Path<i32>
) -> Result<Json<Vec<Beatmap>>, StatusCode> {
    match Beatmap::get_by_beatmapset(&pool, beatmapset_id).await {
        Ok(beatmaps) => Ok(Json(beatmaps)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[utoipa::path(
    get,
    path = "/api/beatmap/random",
    tag = "Beatmap",
    params(
        ("mode" = i32, Query, description = "Game mode"),
        ("status" = String, Query, description = "Beatmap status"),
        ("limit_difficulty" = Option<f64>, Query, description = "Optional difficulty limit")
    ),
    responses(
        (status = 200, description = "Random beatmap found", body = BeatmapSchema),
        (status = 404, description = "No beatmap found")
    ),
    summary = "Get a random beatmap",
    description = "Get a random beatmap based on mode, status and optional difficulty limit"
)]
pub async fn get_random(
    State(pool): State<PgPool>,
    Query(params): Query<RandomBeatmapQuerySchema>,
) -> Result<Json<Beatmap>, StatusCode> {
    info!("Received params - mode: {}, status: {}, difficulty: {:?}", params.mode, params.status, params.limit_difficulty);
    
    // Ensure difficulty is positive if provided
    let limit_difficulty = params.limit_difficulty.and_then(|d: f64| if d <= 0.0 { None } else { Some(d) });
    
    info!("Processed limit_difficulty: {:?}", limit_difficulty);
    
    match Beatmap::get_random_beatmap(&pool, params.mode, &params.status, limit_difficulty).await {
        Ok(Some(beatmap)) => {
            info!("Found beatmap with difficulty: {}", beatmap.difficulty_rating);
            Ok(Json(beatmap))
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

