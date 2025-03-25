use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::feature::user::handler::UserHandler;

use super::service::CandlesService;

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    title: String,
    slug: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UserResponse {
    id: String,
}
#[derive(Clone)]
pub struct CandlesHandler {
    user_service: Arc<CandlesService>,
}

impl CandlesHandler {
    pub fn new(user_service: Arc<CandlesService>) -> Self {
        Self { user_service }
    }
}

#[utoipa::path(
    get,
    path = "/candles",
    responses(
        (status = 201, description = "User created successfully"),
        (status = 400, description = "Bad request")
    ),
    tag = "candles"
)]
pub async fn get_all_candles(
    State(handler): State<Arc<CandlesHandler>>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    "sdad"
}
