use super::{
    entity::CreateCandleRequest,
    service::{CandlesService, CandlesServiceTrait},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

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
    Json(payload): Json<CreateCandleRequest>,
) -> impl IntoResponse {
    if let Err(validation_errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "errors": validation_errors.to_string() })),
        );
    }
    match handler
        .candles_service
        .create_candle(
            &payload.title,
            payload.version,
            payload.price,
            payload.color_id,
        )
        .await
    {
        Ok(candles_id) => (
            StatusCode::CREATED,
            Json(serde_json::json!({"id":candles_id})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ),
    }
}
