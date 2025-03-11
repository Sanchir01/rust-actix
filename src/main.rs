use crate::app::envstate::AppState;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    serve, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use tokio::{main, net};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
#[derive(Serialize, Deserialize, ToSchema)]
struct Item {
    id: i32,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct RequestItem {
    name: String,
    description: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(greet),
    components(schemas(Item)),
    tags(
        (name = "Sample Project", description = "This is sample Mahakala swagger integrations")
    )
)]
struct ApiDoc;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

#[main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Could not load .env file");
    println!("Starting server at http://localhost:5000");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");
    let app_state = AppState { db_pool };

    let listener = net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    let swagger = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());

    let api_routes = Router::new()
        .route("/hello", get(greet).with_state(app_state.clone()))
        .route("/name", get(greet_name))
        .merge(swagger);

    let app = Router::new().nest("/api", api_routes);

    serve(listener, app.into_make_service()).await.unwrap();
    Ok(())
}

#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "Hello World"),
        (status = 404)
    )
)]
async fn greet() -> impl IntoResponse {
    "Hello world"
}

async fn greet_name(Path(id): Path<u32>) -> impl IntoResponse {
    format!("Hello user number {id}")
}
