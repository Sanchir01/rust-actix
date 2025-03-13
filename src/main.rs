use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use log::{info, error, warn, debug};
use sqlx::postgres::PgPool;
use utoipa::ToSchema;
use colored::*;
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
    let _pool = init_primary_db(&config);
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug) // Уровень логирования
        .with_colors(true) // Включаем цветной вывод
        .init()
        .unwrap();

    // Примеры использования логов
    info!("This is an info message");
    error!("This is an error message");
    warn!("This is a warning message");
    debug!("This is a debug message");

    // Использование colored для цветного вывода
    println!("{}", "This is a custom colored message".green());
    run_http_server().await;
    // let swagger = SwaggerUi::new("/swagger-ui/{_:.*}")
    //     .url("/api-docs/openapi.json", ApiDoc::openapi());
    Ok(())
}
