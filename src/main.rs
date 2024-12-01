use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handlers;
mod routes;
mod  models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(|| {
        App::new()            
            .configure(routes::configure)            
            .configure(routes::configure_protected)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await

}