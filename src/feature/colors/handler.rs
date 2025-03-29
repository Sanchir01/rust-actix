use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use super::service::ColorService;

#[derive(Clone)]
pub struct ColorHandler {
    color_service: Arc<ColorService>,
}

impl ColorHandler {
    pub fn new(color_service: Arc<ColorService>) -> Self {
        Self { color_service }
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
pub async fn get_all_candles(State(handler): State<Arc<ColorHandler>>) -> impl IntoResponse {
    "sdasd"
}
