use tokio::net::TcpListener;

use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    serve, Router,
};


pub async fn run_http_server() {
    let listener = TcpListener::bind("0.0.0.0:5000")
        .await
        .expect("Failed to bind port");
    let routers = Router::new().route("/hello", get(greet)).route("/hello/{id}", get(greet_name));
    let app = Router::new().nest("/api", routers);

    serve(listener, app).await.unwrap();
    println!("🚀 Server running on http://localhost:5000");
}

#[utoipa::path(get,path="/api/hello",
    responses((status=OK)) 
    )]
async fn greet() -> impl IntoResponse {
    "Hello world"
}

async fn greet_name(Path(id): Path<u32>) -> impl IntoResponse {
    format!("Hello user number {id}")
}

