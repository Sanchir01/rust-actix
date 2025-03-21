use super::service::UserService;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use simd_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct UserHandler {
    user_service: Arc<Mutex<UserService>>,
}

impl UserHandler {
    pub fn new(user_service: Arc<Mutex<UserService>>) -> Self {
        Self { user_service }
    }

    pub async fn handle_get_hello(State(handler): State<Arc<UserHandler>>) -> impl IntoResponse {
        (StatusCode::OK, "Hello, world!")
    }
}
