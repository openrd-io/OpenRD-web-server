use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

use openRD_web_server::handlers;
use openRD_web_server::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = handlers::db::init_pool();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))            
            .configure(routes::configure) // 不需要 auth 校验的路由
            .configure(routes::configure_protected) // 需要 auth 校验的路由
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
