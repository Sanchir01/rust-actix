use super::service::UserService;
use axum::{extract::State, http::{header, HeaderValue, StatusCode}, response::IntoResponse, routing::head, Json};
use serde::Deserialize;
use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use utoipa::ToSchema;
use validator::Validate;
use crate::feature::user::entity::CreateUserRequest;


#[derive(Deserialize, ToSchema)]
pub struct UserResponse {
    id: String,
}
#[derive(Clone)]
pub struct UserHandler {
    user_service: Arc<UserService>,
}

impl UserHandler {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }
}

#[utoipa::path(
    get,
    path = "/users/hello",
    responses(
        (status = 200, description = "Success response"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn handle_get_hello(State(handler): State<Arc<UserHandler>>) -> impl IntoResponse {
    "Hello, world!"
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
) -> impl IntoResponse {
    if let Err(validation_errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "errors": validation_errors.to_string() })),
        );
    }
    

    match handler
        .user_service
        .create_user(&payload.title, &payload.slug)
        .await
    {

        Ok((user_id,cookies)) => {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(header::SET_COOKIE, HeaderValue::from_str(cookies).unwrap());
            (
       
            StatusCode::CREATED,
            Json(serde_json::json!({ "id": user_id }))
        )},
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ),
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
