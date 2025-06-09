use crate::models::user::{create_user, authenticate_user, CreateUserInput, LoginInput, UserResponse};
use crate::AppState;

use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

pub async fn register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateUserInput>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, Json<serde_json::Value>)> {
    let user = create_user(&app_state.pool, payload)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))))?;

    Ok((StatusCode::CREATED, Json(user.into())))
}

pub async fn login_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginInput>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<serde_json::Value>)> {
    let user = authenticate_user(&app_state.pool, payload)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": e.to_string() }))))?;

    let token = app_state.jwt_service.generate_token(user.id, &user.username)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    Ok((StatusCode::OK, Json(AuthResponse {
        token,
        user: user.into(),
    })))
}
