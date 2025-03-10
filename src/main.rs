use actix_web::{get, App, HttpServer,Responder,web};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize,FromRow)]
struct Item {
    id:i32,
    name:String,
    description:String,
}

#[derive(Deserialize)]
struct RequestItem{
    name:String,
    description:String,
}

#[derive(Clone)]
struct AppState{
    db_pool:PgPool
}

impl AppState{}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Could not load .env file");
    println!("Starting server at http://localhost:8000");

    HttpServer::new(|| {
        App::new().service(greet)
    }).bind("0.0.0.0:8000")?.run().await
}
#[get("/hello")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}