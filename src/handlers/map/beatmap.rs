use crate::models::map::beatmap::{Beatmap, BeatmapSchema};
use crate::models::common::PaginationParams;
use axum::{response::Json, http::StatusCode};
use axum::extract::{State, Query, Path};
use sqlx::PgPool;

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
