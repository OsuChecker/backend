use crate::models::map::beatmapset::{Beatmapset};
use crate::models::common::PaginationParams;
use axum::{response::Json, http::StatusCode};
use axum::extract::{State, Query};
use sqlx::PgPool;

#[utoipa::path(
    get,
    path = "/api/beatmapset",
    tag = "Beatmapset",
    responses(
        (status = 200, description = "Beatmapsets found", body = Vec<Beatmapset>),
        (status = 500, description = "Internal server error")
    ),
    summary = "Get all beatmapsets",
    description = "Get all beatmapsets with pagination"
)]
pub async fn get_beatmapset(
    State(pool): State<PgPool>,
    Query(params): Query<PaginationParams>
) -> Result<Json<Vec<Beatmapset>>, StatusCode> {
    let page = params.get_page();
    let per_page = params.get_per_page();

    match Beatmapset::get_all(&pool, page, per_page).await {
        Ok(beatmaps) => Ok(Json(beatmaps)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}