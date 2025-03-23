use std::sync::Arc;
use tokio::net::TcpListener;

use crate::app::handlers::Handlers;
use crate::feature::user::handler::{get_users, handle_get_hello};
use crate::utils::swagger::setup_swagger;
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
    let swagger = setup_swagger();
    let routers = Router::new()
        .route("/hello/{id}", post(greet_name))
        .route("/users/hello", get(handle_get_hello))
        .route("/users", get(get_users))
        .with_state(user_handlers);

    let app = Router::new().merge(swagger).nest("/api", routers);

    axum::serve(listener, app).await.unwrap();
    println!("🚀 Server running on http://localhost:5000");
}
#[utoipa::path(
    post,
    path = "/users/{id}",
    responses(
        (status = 200, description = "Success response"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("id" = u32, Path, description = "User ID")
    ),
    tag = "users"
)]
pub async fn greet_name(Path(id): Path<u32>) -> impl IntoResponse {}
