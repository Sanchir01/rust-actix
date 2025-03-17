use tokio::net::TcpListener;

use axum::{
    Router,
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    serve::ListenerExt,
};

use crate::feature::user::handler::get_all_users;

pub async fn run_http_server() {
    let listener = TcpListener::bind("0.0.0.0:5000")
        .await
        .unwrap()
        .tap_io(|tcp_stream| {
            if let Err(err) = tcp_stream.set_nodelay(true) {
                eprintln!("Failed to set nodelay: {}", err);
            }
        });

    let routers = Router::new()
        .route("/hello", get(greet))
        .route("/hello/{id}", post(greet_name))
        .route("/test",get(get_all_users) );
    let app = Router::new().nest("/api", routers);

    axum::serve(listener, app).await.unwrap();
    println!("🚀 Server running on http://localhost:5000");
}

async fn greet() -> impl IntoResponse {
    "Hello world"
}

async fn greet_name(Path(id): Path<u32>) -> impl IntoResponse {}
