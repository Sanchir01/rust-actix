use super::service::{UserService, UserServiceTrait};
use crate::{feature::user::entity::CreateUserRequest, utils::errors_message::ErrorMessage};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, ToSchema)]
pub struct UserResponse {
    id: String,
}
#[derive(Clone)]
pub struct UserHandler {
    user_service: Arc<UserService>, // Убираем generic параметр
}

impl UserHandler {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }
}

#[utoipa::path(
    post,
    path = "/api/users",
    responses(
        (status = 201, description = "User created successfully"),
        (status = 400, description = "Bad request")
    ),
    tag = "users"
)]
pub async fn create_user_handler(
    State(handler): State<Arc<UserHandler>>,
    Json(payload): Json<CreateUserRequest>,
) -> Response {
    if let Err(validation_errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "errors": validation_errors.to_string() })),
        )
            .into_response();
    }

    match handler
        .user_service
        .create_user_service(
            &payload.title,
            &payload.slug,
            &payload.email,
            &payload.phone,
            &payload.password,
        )
        .await
    {
        Ok((user_id, refresh_token, access_token)) => {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                header::SET_COOKIE,
                HeaderValue::from_str(&refresh_token.encoded().to_string()).unwrap(),
            );
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&access_token.encoded().to_string()).unwrap(),
            );
            let mut response = Json(serde_json::json!({ "id": user_id })).into_response();
            *response.headers_mut() = headers;
            response.status_mut().clone_from(&StatusCode::CREATED);
            response
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 201, description = "User created successfully"),
        (status = 400, description = "Bad request")
    ),
    tag = "users"
)]
pub async fn get_users(State(handler): State<Arc<UserHandler>>) -> impl IntoResponse {
    match handler.user_service.get_users().await {
        Ok(users) => (StatusCode::OK, Json(users)),
        Err(e) => {
            eprintln!("Error getting users: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/users/:id",
    responses(
        (status = 201, description = "User created successfully"),
        (status = 400, description = "Bad request")
    ),
    tag = "users"
)]
pub async fn get_user_by_id_handler(
    State(handler): State<Arc<UserHandler>>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    match handler.user_service.get_user_by_id_service(user_id).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => {
            eprintln!("Error getting user: {:?}", e);
            let error_response = Json(serde_json::json!({
                "error": e
            }));
            if matches!(e, ErrorMessage::NotFoundUserId) {
                return Err((StatusCode::NOT_FOUND, error_response));
            }
            Err((StatusCode::INTERNAL_SERVER_ERROR, error_response))
        }
    }
}
