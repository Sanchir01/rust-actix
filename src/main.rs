use std::env;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
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

impl AppState {}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Could not load .env file");
    println!("Starting server at http://localhost:8000");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL in not implented");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failded to connect to Postgres");
    let app_state = AppState { db_pool };
    HttpServer::new(|| App::new().service(greet).service(greet_name))
        .bind("0.0.0.0:8000")?
        .workers(4)
        .run()
        .await
}
#[get("/hello")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/greet/{id}")]
async fn greet_name(id: web::Path<u32>) -> impl Responder {
    format!("Hello user number {id}")
}
