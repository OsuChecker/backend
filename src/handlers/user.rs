use sqlx::PgPool;
use crate::models::user::user::User;
use crate::models::common::PaginationParams;
use axum::{response::Json, http::StatusCode};
use axum::extract::{State, Query, Path};


#[utoipa::path(
    get,
    path = "/api/user/{id}",
    tag = "User",
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    ),
    summary = "Get user by id",
    description = "Get a user by their id"
)]
pub async fn get_user_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<User>, StatusCode> {
    match User::get_by_id(&pool, id).await {
        Ok(user) => Ok(Json(user.unwrap())),
        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}   

#[utoipa::path(
    get,
    path = "/api/user",
    tag = "User",
    responses(
        (status = 200, description = "Users found", body = Vec<User>),
        (status = 500, description = "Internal server error")
    ),
    summary = "Get all users",
    description = "Get all users with pagination"
)]
pub async fn get_users(
    State(pool): State<PgPool>,
    Query(params): Query<PaginationParams>
) -> Result<Json<Vec<User>>, StatusCode> {
    let page = params.get_page();
    let per_page = params.get_per_page();

    match User::get_all(&pool, page, per_page).await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
