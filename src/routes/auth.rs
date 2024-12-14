use crate::handlers::auth::generate_token;
use crate::handlers::db::DbPool;
use crate::handlers::error::AppError;
use crate::models::user::User;
use crate::schema::users::dsl::*;
use actix_web::{post, web, HttpResponse};
use bcrypt::verify;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    user_id: i32,
}

#[post("/auth/login")]
async fn login(
    pool: web::Data<DbPool>,
    login_req: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalServerError)?;
    // 查找用户
    let user = users
        .filter(name.eq(&login_req.username))
        .first::<User>(&mut conn)
        .map_err(|_| AppError::Unauthorized)?;

    // 验证密码
    if !verify(&login_req.password, &user.password).map_err(|_| AppError::Unauthorized)? {
        return Err(AppError::Unauthorized);
    }

    // 根据用户类型分配角色
    let role = if user.email == "a11mypr1nt@gmail.com" {
        "ADMIN"
    } else {
        "USER"
    };

    // 生成带角色的 token
    let token = generate_token(&user.id.to_string(), &role)?;

    // 返回响应
    Ok(HttpResponse::Ok().json(LoginResponse {
        token,
        user_id: user.id,
    }))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}
