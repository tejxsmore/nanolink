use axum::{
    extract::{Request, State, Extension},
    http::{StatusCode, HeaderMap},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::auth::jwt::JwtService;

#[derive(Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
}

pub async fn optional_auth_middleware(
    State(jwt_service): State<JwtService>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if let Ok(claims) = jwt_service.verify_token(token) {
                    if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                        let auth_user = AuthUser {
                            id: user_id,
                            username: claims.username,
                        };
                        request.extensions_mut().insert(auth_user);
                    }
                }
            }
        }
    }
    
    next.run(request).await
}

pub async fn auth_middleware(
    State(jwt_service): State<JwtService>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if let Ok(claims) = jwt_service.verify_token(token) {
                    if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                        let auth_user = AuthUser {
                            id: user_id,
                            username: claims.username,
                        };
                        request.extensions_mut().insert(auth_user);
                        return Ok(next.run(request).await);
                    }
                }
            }
        }
    }
    
    Err(StatusCode::UNAUTHORIZED)
}
