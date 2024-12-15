use crate::handlers::db::DbPool;
use crate::handlers::error::AppError;
use crate::models::user::{UpdateUserDTO, User, UserDTO};
use crate::utils::api_response::ApiResponse;
use crate::{log_error, log_info}; // 导入日志宏
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web_grants::protect;

#[get("/users/{id}")]
#[protect("USER")]
pub async fn get_user(
    pool: web::Data<DbPool>,
    biz_id: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let user_id = biz_id.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || User::find_by_id(&mut conn, &user_id))
        .await
        .map_err(|e| {
            log_error!("failed to get user :{}", e);
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
        log_error!("Failed to get DB connection: {}", e);
        AppError::InternalServerError
    })?;
    let user = web::block(move || User::create(&mut conn, &user_dto))
        .await
        .map_err(|e| {
            log_error!("Failed to create user: {}", e);
            AppError::InternalServerError
        })?
        .map_err(AppError::from)?;

    log_info!("Successfully created user with id: {}", user.id);

    Ok(HttpResponse::Ok().json(ApiResponse::success(user.biz_id)))
}

#[put("/users/{id}")]
#[protect("USER")]
pub async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
    user_dto: web::Json<UpdateUserDTO>,
) -> Result<HttpResponse, AppError> {
    let user_id = user_id.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let user_id_clone = user_id.clone();

    let user = web::block(move || User::update(&mut conn, &user_id_clone, &user_dto))
        .await
        .map_err(|e| {
            log_error!("failed to update user,request id={},error={}", user_id, e);
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
