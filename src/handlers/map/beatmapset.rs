use crate::models::map::beatmapset::{Beatmapset, BeatmapsetSchema};
use crate::models::common::PaginationParams;
use axum::{
    response::Json,
    http::StatusCode,
    extract::{State, Query, Path},
};
use sqlx::PgPool;

#[utoipa::path(
    get,
    path = "/api/beatmapsets",
    tag = "Beatmapsets",
    params(
        ("page" = Option<i64>, Query, description = "Page number, default: 1"),
        ("per_page" = Option<i64>, Query, description = "Items per page, default: 20")
    ),
    responses(
        (status = 200, description = "Beatmapsets found", body = Vec<BeatmapsetSchema>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_beatmapsets(
    State(pool): State<PgPool>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<BeatmapsetSchema>>, StatusCode> {
    let page = params.get_page();
    let per_page = params.get_per_page();
    
    let beatmapsets = Beatmapset::get_all(&pool, page, per_page)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Convertir les Beatmapset en BeatmapsetSchema
    let schema_beatmapsets = beatmapsets.into_iter()
        .map(|beatmapset| beatmapset.to_schema())
        .collect();
    
    Ok(Json(schema_beatmapsets))
}

#[utoipa::path(
    get,
    path = "/api/beatmapsets/{id}",
    tag = "Beatmapsets",
    params(
        ("id" = i32, Path, description = "Beatmapset ID")
    ),
    responses(
        (status = 200, description = "Beatmapset found", body = BeatmapsetSchema),
        (status = 404, description = "Beatmapset not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_beatmapset_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<BeatmapsetSchema>, StatusCode> {
    let beatmapset = Beatmapset::get_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(beatmapset.to_schema()))
}