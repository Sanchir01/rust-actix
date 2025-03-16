use crate::app::config::Config;
use crate::app::db::init_primary_db;
use crate::servers::http::server::run_http_server;
use axum::{extract::State, http::StatusCode, response::Json as AxumResponseJson, Json};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use slog::{Drain, Logger, info, o};
use slog_async::Async;
use slog_scope::{GlobalLoggerGuard, logger, set_global_logger};
use slog_term::{CompactFormat, TermDecorator};
use sqlx::{postgres::PgPool, prelude::FromRow,query_as};
use std::sync::Mutex;
use uuid::Uuid;
mod app;
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
#[cfg(target_os = "windows")]
use mimalloc::MiMalloc as GlobalAlloc;
#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc as GlobalAlloc;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

#[derive(Debug, Serialize, FromRow)]
struct User {
    id: Uuid,
    name: String,
    slug: String,
}

#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;

impl AppState {
    pub async fn create_user_impl(&self,title:&str) -> Result<User, sqlx::Error> {
       let query = r#"
            INSERT INTO users (name, slug)
            VALUES ($1, $2)
            RETURNING id, name, slug
        "#;
        let user:(Uuid, String,String) = query_as( query)
        .bind(title)
        .bind(title)
        .fetch_one(&self.db_pool).
        await?;
            
        Ok(User{
            id: user.0,
            name: user.1,
            slug: user.2,
        })
      
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Could not load .env file");

    let config = Config::new().await;

    println!("Config: {:?}", config);
    let _pool = init_primary_db(&config).await.expect("Could not initialize database");

    let _guard = init_logger();

    info!(logger(), "Приложение запущено");

    run_http_server().await;

    println!("Starting server at http://localhost:5000");

    // let swagger = SwaggerUi::new("/swagger-ui/{_:.*}")
    //     .url("/api-docs/openapi.json", ApiDoc::openapi());
    Ok(())
}
async fn create_user(
    State(app_state): State<AppState>, 
    Json(payload): Json<User>
) -> Result<(StatusCode, AxumResponseJson<User>), StatusCode> {
    let item = app_state.create_user_impl(&payload.name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, AxumResponseJson(item)))
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

