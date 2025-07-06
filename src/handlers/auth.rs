use axum::{
    response::Json,
    extract::Extension,
};
use crate::models::user::user::User;

pub async fn me(
    Extension(user): Extension<User>
) -> Json<User> {
    Json(user)
} 