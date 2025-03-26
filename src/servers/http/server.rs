use axum::middleware::from_fn;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::app::handlers::Handlers;
use crate::feature::user::handler::{create_user_handler, get_users, handle_get_hello};
use crate::servers::http::middleware::auth_middleware;
use crate::utils::swagger::setup_swagger;
use axum::{
    Router,
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
    let _swagger = setup_swagger();

    let public_routes = Router::new().route("/login", post(create_user_handler));

    let protected_routes = Router::new()
        .route("/users", get(get_users))
        .layer(from_fn(auth_middleware));

    let app = Router::new()
        .nest("/api", public_routes)
        .nest("/api", protected_routes)
        .layer(get_cors())
        .with_state(user_handlers);
    axum::serve(listener, app).await.unwrap();
    println!("🚀 Server running on http://localhost:5000");
}

fn get_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
