use actix_web::{web, Scope};
use crate::routes::{user, auth, chat};

// API 版本前缀常量
pub const API_VERSION: &str = "/api/v1";

// 配置所有 API 路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(API_VERSION)
            .configure(auth::configure_routes)  // 认证相关路由
            .configure(user::configure_routes)  // 用户相关路由
            .configure(chat::configure_routes)  // 聊天相关路由
    );
} 