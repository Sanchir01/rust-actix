use axum::{extract::Request, middleware::Next, response::IntoResponse};
use axum::http::HeaderMap;

pub async fn auth_middleware(request: Request, next: Next) -> impl IntoResponse {
    let headers: &HeaderMap = request.headers();
    
    if let Some(auth_header) = headers.get("Authorization") {
        println!("Auth header: {:?}", auth_header);
    }
    
    let response = next.run(request).await;
    response
}
