use axum::{extract::Request, middleware::Next, response::IntoResponse};

pub async fn hello_mid(request: Request, next: Next) -> impl IntoResponse {
    println!("hello from middleware!");
    let response = next.run(request).await;
    response
}
