use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use sqlx::postgres::PgPool;
use utoipa::ToSchema;

use crate::app::db::init_primary_db;
use crate::config::Config;
use crate::servers::http::server::run_http_server;
use slog::{info, o, Drain, Logger};
use slog_async::Async;
use slog_scope::{set_global_logger, GlobalLoggerGuard,logger};
use slog_term::{CompactFormat, TermDecorator};
use std::sync::Mutex;
mod servers;
mod config;
mod app;




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
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Could not load .env file");

    println!("Starting server at http://localhost:5000");

    let config = Config::new().await;

    println!("Config: {:?}", config);
    let _pool = init_primary_db(&config);

    let _guard = init_logger(); // Устанавливаем глобальный логгер

    info!(logger(),"Приложение запущено"); // Логирование через slog
    log::info!("Привет, мир!");

    run_http_server().await;
    // let swagger = SwaggerUi::new("/swagger-ui/{_:.*}")
    //     .url("/api-docs/openapi.json", ApiDoc::openapi());
    Ok(())
}


