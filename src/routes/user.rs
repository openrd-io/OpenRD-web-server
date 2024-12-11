use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use crate::models::user::{User, UserDTO};
use crate::handlers::db::DbPool;
use diesel::prelude::*;

// 创建用户
#[post("/create")]
async fn create_user(pool: web::Data<DbPool>, user: web::Json<UserDTO>) -> impl Responder {
    let conn = pool.get().expect("无法获取数据库连接");
    let new_user = user.into_inner();

    let result = web::block(move || {
        User::create(&conn, &new_user)
    }).await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => HttpResponse::InternalServerError().body("创建用户时出错"),
        Err(_) => HttpResponse::InternalServerError().body("服务器错误"),
    }
}

// 获取用户
#[get("/get/{id}")]
async fn get_user(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();
    let conn = pool.get().expect("无法获取数据库连接");

    let result = web::block(move || {
        User::get_by_id(&conn, user_id)
    }).await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => HttpResponse::NotFound().body("未找到用户"),
        Err(_) => HttpResponse::InternalServerError().body("服务器错误"),
    }
}

// 更新用户
#[put("/update/{id}")]
async fn update_user(pool: web::Data<DbPool>, path: web::Path<i32>, user: web::Json<UserDTO>) -> impl Responder {
    let user_id = path.into_inner();
    let conn = pool.get().expect("无法获取数据库连接");
    let updated_user = user.into_inner();

    let result = web::block(move || {
        User::update(&conn, user_id, &updated_user)
    }).await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => HttpResponse::InternalServerError().body("更新用户时出错"),
        Err(_) => HttpResponse::InternalServerError().body("服务器错误"),
    }
}

// 删除用户
#[delete("/delete/{id}")]
async fn delete_user(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();
    let conn = pool.get().expect("无法获取数据库连接");

    let result = web::block(move || {
        User::delete(&conn, user_id)
    }).await;

    match result {
        Ok(Ok(_)) => HttpResponse::Ok().body("用户已删除"),
        Ok(Err(_)) => HttpResponse::InternalServerError().body("删除用户时出错"),
        Err(_) => HttpResponse::InternalServerError().body("服务器错误"),
    }
}