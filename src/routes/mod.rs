use actix_web::{web, App, HttpResponse, Responder};
use crate::handlers::auth::Auth;

mod auth;
    App::new()
        .wrap(Auth)
        .configure(configure)


fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(hello)));
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}