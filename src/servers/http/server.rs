
pub async fn run_http_server() {
    let listener = TcpListener::bind("0.0.0.0:5000")
        .await
        .expect("Failed to bind port");

    let app = create_router();
    Router::new().nest("/api");

    serve(listener, app).await.unwrap();
    println!("🚀 Server running on http://localhost:5000");
}
async fn greet() -> impl IntoResponse {
    "Hello world"
}

async fn greet_name(Path(id): Path<u32>) -> impl IntoResponse {
    format!("Hello user number {id}")
}