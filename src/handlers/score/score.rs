use crate::models::score::score::{Score, ScoreSchema};
use crate::models::common::PaginationParams;
use axum::{response::Json, http::StatusCode};
use axum::extract::{State, Query, Path};
use sqlx::PgPool;
use tracing::info;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Debug, IntoParams, ToSchema, Validate)]
pub struct LeaderboardParams {  
    #[validate(range(min = 1))]
    pub page: Option<i64>,
    #[validate(range(min = 1, max = 50))]
    pub per_page: Option<i64>,
    pub mods: Option<i32>,
}


#[utoipa::path(
    get,
    path = "/api/leaderboard/{beatmap_id}",
    tag = "Score",
    params(LeaderboardParams),
    responses(
        (status = 200, description = "Leaderboard retrieved successfully", body = Vec<ScoreSchema>),
        (status = 404, description = "Beatmap not found")
    ),
    summary = "Get leaderboard",
    description = "Get leaderboard for a beatmap"
)]
pub async fn get_leaderboard(
    State(pool): State<PgPool>,
    Query(params): Query<LeaderboardParams>,
    Path(beatmap_id): Path<i32>,
) -> Result<Json<Vec<Score>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let leaderboard = Score::get_leaderboard(&pool, beatmap_id, params.mods, page, per_page).await;

    match leaderboard {
        Ok(leaderboard) => Ok(Json(leaderboard)),
        Err(err) => {
            info!("Error getting leaderboard: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}