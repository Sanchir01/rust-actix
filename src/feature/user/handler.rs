use super::service::UserService;
use axum::{extract::State, response::IntoResponse};

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

pub async fn handle_get_hello(State(handler): State<Arc<UserHandler>>) -> impl IntoResponse {
    "Hello, world!"
}
