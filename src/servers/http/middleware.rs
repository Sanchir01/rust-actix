use axum::Json;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use axum::{extract::Request, middleware::Next, response::IntoResponse};

use crate::feature::user::jwt::parse_token;

fn extract_token(auth_header: &str) -> Result<&str, &'static str> {
    auth_header
        .strip_prefix("Bearer ")
        .ok_or("Invalid bearer token format")
        .map(|token| token.trim())
}

pub async fn auth_middleware(request: Request, next: Next) -> Response {
    let headers: &HeaderMap = request.headers();

    let auth_header = match headers.get("Authorization") {
        Some(header) => match header.to_str() {
            Ok(header_str) if header_str.starts_with("Bearer") => header_str,
            _ => {
                return (
                    StatusCode::FORBIDDEN,
                    Json(serde_json::json!({"error":"Invalid headers"})),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({ "error": "Missing authorization header" })),
            )
                .into_response();
        }
    };
    let token = match extract_token(auth_header) {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({"error":"Invalid headers"})),
            )
                .into_response();
        }
    };

    println!("token auth {}", token);
    let data = match parse_token(token) {
        Ok(data) => data,
        Err(_) => {
            return (
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({"error":"Invalid token"})),
            )
                .into_response();
        }
    };
    println!("decode token {:?}", data);
    let response = next.run(request).await;
    response
}
