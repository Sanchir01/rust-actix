use super::service::UserService;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use simd_json::json;

pub struct UserHandler {
    user_service: UserService,
}

impl UserHandler {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    pub async fn get_all_users(&self) -> Result<Response, Response> {
        // Получаем пользователей из сервиса
        let users = self.user_service.get_all_users().await.map_err(|err| {
            // Преобразуем ошибку сервиса в ответ Axum
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response() // Преобразуем в Response
        })?;

        Ok((StatusCode::OK, Json(json!(users))).into_response())
    }
}

pub fn get_all_users(){
    println!("test");
}
