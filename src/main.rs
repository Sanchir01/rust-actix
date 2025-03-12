use axum::extract::{Path, State};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use utoipa::ToSchema;
mod servers;
use crate::servers::http::server::run_http_server;
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

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Could not load .env file");
    println!("Starting server at http://localhost:5000");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");
    let app_state = AppState { db_pool };
    run_http_server();
    // let swagger = SwaggerUi::new("/swagger-ui/{_:.*}")
    //     .url("/api-docs/openapi.json", ApiDoc::openapi());
    Ok(())
}
