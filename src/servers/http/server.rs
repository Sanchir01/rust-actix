use std::sync::Arc;
use tokio::net::TcpListener;

use crate::app::handlers::Handlers;
use crate::feature::user::handler::handle_get_hello;
use axum::{
    Router,
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    serve::ListenerExt,
};

pub async fn run_http_server(handlers: Arc<Handlers>) {
    let listener = TcpListener::bind("0.0.0.0:5000")
        .await
        .unwrap()
        .tap_io(|tcp_stream| {
            if let Err(err) = tcp_stream.set_nodelay(false) {
                eprintln!("Failed to set nodelay: {}", err);
            }
        });

    let user_handlers = handlers.users_handler.clone();
    let routers = Router::new()
        .route("/hello", get(greet))
        .route("/hello/{id}", post(greet_name))
        .route("/users/hello", get(handle_get_hello))
        .with_state(user_handlers);

    let app = Router::new().nest("/api", routers);

    axum::serve(listener, app).await.unwrap();
    println!("🚀 Server running on http://localhost:5000");
}

async fn greet() -> impl IntoResponse {
    "Hello world"
}

async fn greet_name(Path(id): Path<u32>) -> impl IntoResponse {}
