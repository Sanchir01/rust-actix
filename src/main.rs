use crate::app::config::Config;
use crate::app::db::init_primary_db;
use crate::servers::http::server::run_http_server;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use slog::{Drain, Logger, info, o};
use slog_async::Async;
use slog_scope::{GlobalLoggerGuard, logger, set_global_logger};
use slog_term::{CompactFormat, TermDecorator};
use sqlx::prelude::FromRow;
use std::sync::Mutex;
use uuid::Uuid;
mod app;
mod feature;
mod servers;

#[derive(Serialize, Deserialize)]
struct Item {
    id: Uuid,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct RequestItem {
    name: String,
    description: String,
}
#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc as GlobalAlloc;
#[cfg(target_os = "windows")]
use mimalloc::MiMalloc as GlobalAlloc;

#[derive(Debug, Serialize, FromRow)]
struct User {
    id: Uuid,
    name: String,
    slug: String,
}

#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Could not load .env file");

    let config = Config::new().await;

    println!("Config: {:?}", config);
    let pool = init_primary_db(&config)
        .await
        .expect("Could not initialize database");

    let _guard = init_logger();

    info!(logger(), "Приложение запущено");
    info!(logger(), "Starting server at http://localhost:5000");
    run_http_server(pool).await;
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

    slog_stdlog::init().expect("Failed to initialize slog_stdlog");

    guard
}
