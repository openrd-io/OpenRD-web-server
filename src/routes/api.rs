use crate::{
    routes::{auth, chat, user}, utils::api_response::AppResp,
};
use actix_web::{get, web, HttpResponse, Responder, Scope};

// API 版本前缀常量
pub const API_VERSION: &str = "/api/v1";

// 定义处理函数
#[get("/hello")]
async fn hello_world() -> impl Responder {
    AppResp(Ok("Hello, World!"))
}

// 配置所有 API 路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(API_VERSION)
            .configure(auth::configure_routes) // 认证相关路由
            .configure(user::configure_routes) // 用户相关路由
            .configure(chat::configure_routes), // 聊天相关路由
    );
    cfg.service(hello_world);
}
