use sqlx::Pool;
use sqlx::Postgres;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

use axum::{
    Router,
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    serve::ListenerExt,
};

use crate::feature::user::{
    handler::UserHandler, repository::UserRepository, service::UserService,
};

pub async fn run_http_server(db: Pool<Postgres>) {
    let user_repository = Arc::new(Mutex::new(UserRepository::new(db)));
    let user_service = Arc::new(Mutex::new(UserService::new(user_repository)));
    let user_handler = Arc::new(UserHandler::new(user_service.clone()));

    let listener = TcpListener::bind("0.0.0.0:5000")
        .await
        .unwrap()
        .tap_io(|tcp_stream| {
            if let Err(err) = tcp_stream.set_nodelay(false) {
                eprintln!("Failed to set nodelay: {}", err);
            }
        });

    let routers = Router::new()
        .route("/hello", get(greet))
        .route("/hello/{id}", post(greet_name))
        .route("/users", get(UserHandler::handle_get_hello))
        .with_state(user_handler);

    let app = Router::new().nest("/api", routers);

    axum::serve(listener, app).await.unwrap();
    println!("🚀 Server running on http://localhost:5000");
}

async fn greet() -> impl IntoResponse {
    "Hello world"
}

async fn greet_name(Path(id): Path<u32>) -> impl IntoResponse {}
