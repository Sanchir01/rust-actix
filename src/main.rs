use crate::app::config::Config;
use crate::app::db::init_primary_db;
use crate::servers::http::server::run_http_server;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use slog::{Drain, Logger, info, o};
use slog_async::Async;
use slog_scope::{GlobalLoggerGuard, logger, set_global_logger};
use slog_term::{CompactFormat, TermDecorator};
use sqlx::{postgres::PgPool, prelude::FromRow};
use std::sync::Mutex;
use utoipa::ToSchema;
mod app;
mod servers;

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

#[derive(Debug, Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Could not load .env file");

    let config = Config::new().await;

    println!("Config: {:?}", config);
    let pool = init_primary_db(&config);

    let _guard = init_logger();

    info!(logger(), "Приложение запущено");

    run_http_server().await;

    println!("Starting server at http://localhost:5000");

    // let swagger = SwaggerUi::new("/swagger-ui/{_:.*}")
    //     .url("/api-docs/openapi.json", ApiDoc::openapi());
    Ok(())
}

fn init_logger() -> GlobalLoggerGuard {
    let decorator = TermDecorator::new().build();
    let drain = CompactFormat::new(decorator).build().fuse();
    let drain = Mutex::new(drain).map(slog::Fuse);
    let drain = Async::new(drain).build().fuse();

    let log = Logger::root(drain, o!());

    let guard = set_global_logger(log);

    // Инициализируем `slog` как стандартный логгер (Только ОДИН раз!)
    slog_stdlog::init().expect("Failed to initialize slog_stdlog");

    guard
}
async fn fetch_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    // Выполняем SQL-запрос
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM user")
        .fetch_all(pool)
        .await?;

    Ok(users)
}
