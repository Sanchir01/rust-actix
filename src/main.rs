use std::fmt::format;
use actix_web::{get, App, HttpServer, Responder, web, HttpResponse};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use diesel::{pg::PgConnection,r2d2::{ Pool, self,ConnectionManager} ,prelude::*};


#[derive(Queryable,Serialize)]
struct Item {
    id:i32,
    name:String,
    description:String,
}

#[derive(Serialize,Deserialize)]
struct RequestItem{
    name:String,
    description:String,
}

#[derive(Clone)]
struct AppState{
    db_pool:Pool<ConnectionManager<PgConnection>>
}

impl AppState{}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Could not load .env file");
    println!("Starting server at http://localhost:8000");

    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(greet_name)
    })
        .bind("0.0.0.0:8000")?
        .workers(4).run().await
}
#[get("/hello")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/greet/{id}")]
async fn greet_name(id: web::Path<u32>) -> impl Responder {
    format!("Hello user number {id}")
}