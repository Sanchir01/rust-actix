use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use sqlx::postgres::PgPool;
use utoipa::ToSchema;
use slog::{Drain, Logger, o, info,warn };
use slog_async;
use slog_async::Async;
use slog_scope::{set_global_logger, GlobalLoggerGuard};
use slog_stdlog::StdLog;
use slog_term::{CompactFormat, TermDecorator};
mod servers;
use crate::servers::http::server::run_http_server;
mod config;
use crate::config::Config;
mod app;
mod logger;
use crate::logger::init_logger;
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


    let _guard =init_logger();

    info!(logger(), "🚀 Приложение запущено!");
    warn!(logger(), "⚠️  Предупреждение!");






    run_http_server().await;
    // let swagger = SwaggerUi::new("/swagger-ui/{_:.*}")
    //     .url("/api-docs/openapi.json", ApiDoc::openapi());
    Ok(())
}

/// Инициализация глобального логгера
