use actix_web::web;
use user::{create_user, delete_user, update_user};

use crate::handlers::auth;

mod hello;
mod user;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(hello::hello)
            .service(create_user)
            .service(update_user)
            .service(delete_user),
    );
}

pub fn configure_protected(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .wrap(auth::Auth)
            .service(user::get_user)
            .service(user::create_user)
            // .service(auth)
    );
}
