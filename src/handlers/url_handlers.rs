use crate::models::url_mapping::{insert_url_mapping, get_url_by_code, get_user_urls, CreateUrlInput};
use crate::auth::middleware::AuthUser;
use crate::AppState;

use axum::{
    extract::{Path, State, Extension},
    Json,
    response::{Redirect, IntoResponse},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub original_url: String,
    pub short_code: Option<String>,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
}

pub async fn shorten_handler(
    State(app_state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    Json(payload): Json<ShortenRequest>,
) -> Result<(StatusCode, Json<ShortenResponse>), (StatusCode, Json<serde_json::Value>)> {
    let user_id = auth_user.map(|ext| ext.0.id);
        
    let input = CreateUrlInput {
        original_url: payload.original_url,
        short_code: payload.short_code,
        user_id,
    };

    let mapping = insert_url_mapping(&app_state.pool, input)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))))?;

    let short_url = format!("{}/{}", app_state.base_url, mapping.short_code);
    Ok((StatusCode::CREATED, Json(ShortenResponse { short_url })))
}

pub async fn get_my_urls_handler(
    State(app_state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
) -> Result<Json<Vec<crate::models::url_mapping::UrlMapping>>, (StatusCode, Json<serde_json::Value>)> {
    let user = auth_user.ok_or_else(|| 
        (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Authentication required" })))
    )?;

    let urls = get_user_urls(&app_state.pool, user.0.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    Ok(Json(urls))
}

pub async fn delete_url_handler(
    State(app_state): State<AppState>,
    Path(short_code): Path<String>,
    auth_user: Option<Extension<AuthUser>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    println!("Delete request received for short_code: {}", short_code); // Debug log
    
    let user = auth_user.ok_or_else(|| 
        (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Authentication required" })))
    )?;

    println!("Authenticated user ID: {}", user.0.id); // Debug log

    // First, let's check if the URL exists and belongs to the user
    let check_result = sqlx::query("SELECT id FROM url_mappings WHERE short_code = $1 AND user_id = $2")
        .bind(&short_code)
        .bind(user.0.id)
        .fetch_optional(&app_state.pool)
        .await
        .map_err(|e| {
            println!("Check query error: {}", e); // Debug log
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
        })?;

    if check_result.is_none() {
        println!("URL not found or access denied for short_code: {}", short_code); // Debug log
        return Err((StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "URL not found or access denied" }))));
    }

    // Now delete the URL using short_code
    let result = sqlx::query("DELETE FROM url_mappings WHERE short_code = $1 AND user_id = $2")
        .bind(&short_code)
        .bind(user.0.id)
        .execute(&app_state.pool)
        .await
        .map_err(|e| {
            println!("Delete query error: {}", e); // Debug log
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
        })?;

    println!("Rows affected: {}", result.rows_affected()); // Debug log

    if result.rows_affected() == 0 {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Failed to delete URL" }))));
    }

    println!("URL deleted successfully for short_code: {}", short_code); // Debug log
    Ok(Json(serde_json::json!({ "message": "URL deleted successfully" })))
}

pub async fn redirect_handler(
    State(app_state): State<AppState>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    match get_url_by_code(&app_state.pool, &code).await {
        Ok(Some(mapping)) => {
            // Increment click count
            let _ = sqlx::query("UPDATE url_mappings SET click_count = click_count + 1 WHERE short_code = $1")
                .bind(&code)
                .execute(&app_state.pool)
                .await;
                        
            Redirect::temporary(&mapping.original_url).into_response()
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Short code not found").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        ).into_response(),
    }
}
