use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use super::service::{ColorService, ColorServiceTrait};

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
    path = "/api/colors",
    responses(
        (status = 201, description = "User created successfully"),
        (status = 400, description = "Bad request")
    ),
    tag = "colors"
)]
pub async fn get_all_colors_handler(State(handler): State<Arc<ColorHandler>>) -> impl IntoResponse {
    match handler.color_service.get_all_colors_service().await {
        Ok(users) => (StatusCode::OK, Json(users)),
        Err(e) => {
            eprintln!("Error getting users: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}
