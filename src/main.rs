use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::env;
use utoipa::ToSchema;
mod servers;
use crate::servers::http::server::run_http_server;
mod config;
use crate::config::Config;
mod app;
use crate::app::db::init_primary_db;

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

    let config = Config::new().await;

    println!("Config: {:?}", config);
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    println!("db url {database_url}");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");
    let pool = init_primary_db(&config).await;
    run_http_server().await;
    // let swagger = SwaggerUi::new("/swagger-ui/{_:.*}")
    //     .url("/api-docs/openapi.json", ApiDoc::openapi());
    Ok(())
}


