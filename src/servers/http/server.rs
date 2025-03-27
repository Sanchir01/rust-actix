use axum::middleware::{self};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::app::handlers::Handlers;
use crate::feature::candles::handler::get_all_candles;
use crate::feature::user::handler::{create_user_handler, get_users};
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

    let _swagger = setup_swagger();

    let middleware_builder = ServiceBuilder::new().layer(middleware::from_fn(auth_middleware));

    let auth_routes = Router::new()
        .route("/login", post(create_user_handler))
        .with_state(handlers.users_handler.clone());

    let candles_routes = Router::new()
        .route("/candles", get(get_all_candles))
        .with_state(handlers.candles_handler.clone());

    let protected_routes = Router::new()
        .route("/users", get(get_users))
        .layer(middleware_builder)
        .with_state(handlers.users_handler.clone());

    let public_routes = Router::new().merge(candles_routes).merge(auth_routes);
    let app = Router::new()
        .nest("/api", public_routes)
        .nest("/api/private", protected_routes)
        .layer(get_cors());

    println!("🚀 Server running on http://localhost:5000");

    axum::serve(listener, app).await.unwrap();
}

fn get_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
