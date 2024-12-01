use actix_web::{get, post, web::{self}, HttpResponse, Responder};
use crate::handlers;

use crate::models::user::User;


#[post("/create")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.into_inner())
}

#[get("/get/{id}")]
async fn get_user(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    let user = User::get_by_id(user_id);
    HttpResponse::Ok().json(user)
}