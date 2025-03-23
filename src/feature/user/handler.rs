use super::service::UserService;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use std::sync::Arc;

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
    get,
    path = "/users",
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
