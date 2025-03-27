use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;

use super::service::{CandlesService, CandlesServiceTrait};

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
    candles_service: Arc<CandlesService>,
}

impl CandlesHandler {
    pub fn new(candles_service: Arc<CandlesService>) -> Self {
        Self { candles_service }
    }
}

#[utoipa::path(
    get,
    path = "/api/candles",
    responses(
        (status = 201, description = "User created successfully"),
        (status = 400, description = "Bad request")
    ),
    tag = "candles"
)]
pub async fn get_all_candles(State(handler): State<Arc<CandlesHandler>>) -> impl IntoResponse {
    match handler.candles_service.get_all_candles().await {
        Ok(candles) => (StatusCode::OK, Json(candles)),
        Err(e) => {
            eprintln!("Error getting users: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/candles",
    responses(
        (status = 201, description = "User created successfully"),
        (status = 400, description = "Bad request")
    ),
    tag = "candles"
)]
pub async fn create_candle_handler(
    State(handler): State<Arc<CandlesHandler>>,
) -> impl IntoResponse {
    "create candle "
}
