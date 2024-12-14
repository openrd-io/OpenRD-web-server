use crate::handlers::db::DbPool;
use crate::handlers::error::AppError;
use crate::models::user::{User, UserDTO};
use crate::{app_error, app_info}; // 导入日志宏
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web_grants::protect;

#[get("/users/{id}")]
#[protect("USER")]
pub async fn get_user(
    pool: web::Data<DbPool>,
    biz_id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = biz_id.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || User::find_by_id(&mut conn, &user_id))
        .await
        .map_err(|e| {
            app_error!("failed to get user :{}", e);
            AppError::InternalServerError
        })?
        .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    user_dto: web::Json<UserDTO>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|e| {
        app_error!("Failed to get DB connection: {}", e);
        AppError::InternalServerError
    })?;

    let user = web::block(move || User::create(&mut conn, &user_dto))
        .await
        .map_err(|e| {
            app_error!("Failed to create user: {}", e);
            AppError::InternalServerError
        })?
        .map_err(AppError::from)?;

    app_info!("Successfully created user with id: {}", user.id);
    Ok(HttpResponse::Created().json(user))
}

#[put("/users/{id}")]
#[protect("USER")]
pub async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
    user_dto: web::Json<UserDTO>,
) -> Result<HttpResponse, AppError> {
    let user_id = user_id.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || User::update(&mut conn, user_id, &user_dto))
        .await
        .map_err(|e| {
            app_error!("failed to update user,request id={},error={}", user_id, e);
            AppError::InternalServerError
        })?
        .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
#[protect("USER")]
pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user_id = user_id.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    web::block(move || User::delete(&mut conn, user_id))
        .await
        .map_err(|_| AppError::InternalServerError)?
        .map_err(AppError::from)?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/users")]
#[protect("USER")]
pub async fn list_users(
    pool: web::Data<DbPool>,
    query: web::Query<ListUsersQuery>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let users = web::block(move || {
        User::list(
            &mut conn,
            query.page.unwrap_or(1),
            query.per_page.unwrap_or(10),
        )
    })
    .await
    .map_err(|_| AppError::InternalServerError)?
    .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(users))
}

#[derive(serde::Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user)
        .service(get_user)
        .service(update_user)
        .service(delete_user)
        .service(list_users);
}
