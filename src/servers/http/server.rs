use axum::middleware::from_fn;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::app::handlers::Handlers;
use crate::feature::user::handler::{create_user_handler, get_users, handle_get_hello};
use crate::servers::http::middleware::hello_mid;
use crate::utils::swagger::setup_swagger;
use axum::{Router, routing::get, serve::ListenerExt};
use tower::ServiceBuilder;
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
    let routers = Router::new()
        .route("/users/hello", get(handle_get_hello))
        .route("/users", get(get_users).post(create_user_handler))
        .with_state(user_handlers)
        .layer(get_cort())
        .layer(from_fn(hello_mid));

    let app = Router::new().nest("/api", routers);

    axum::serve(listener, app).await.unwrap();
    println!("🚀 Server running on http://localhost:5000");
}

fn get_cort() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
