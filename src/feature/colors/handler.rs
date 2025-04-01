use super::entity::CreateColorRequest;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::{Value, json};
use std::sync::Arc;
use validator::{Validate, ValidationErrors};


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
pub async fn get_all_color_handler(State(handler): State<Arc<ColorHandler>>) -> impl IntoResponse {
    match handler.color_service.get_all_color_service().await {
        Ok(data) => (StatusCode::OK, Json(data)),
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
pub async fn create_color_handler(
    State(handler): State<Arc<ColorHandler>>,
    Json(payload): Json<CreateColorRequest>,
) -> impl IntoResponse {
    if let Err(validation_errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "errors": format_validation_errors(&validation_errors) })),
        );
    };

    match handler
        .color_service
        .create_color_service(&payload.title, payload.version)
        .await
    {
        Ok(color_id) => (
            StatusCode::CREATED,
            Json(json!({"id":color_id.to_string()})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ),
    }
}

fn format_validation_errors(errors: &ValidationErrors) -> Value {
    let mut error_map = serde_json::Map::new();

    for (field, errs) in errors.field_errors() {
        let messages: Vec<String> = errs
            .iter()
            .filter_map(|e| e.message.as_ref().map(|msg| msg.to_string()))
            .collect();
        error_map.insert(field.to_string(), json!(messages));
    }

    json!(error_map)
}

