use actix_web::{web,Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(get_users))
            .route(web::post().to(create_user)),
    );
}

async fn get_users() -> impl Responder {
    // 获取用户的处理逻辑

}

async fn create_user(user: &mut User) -> impl Responder {
    // 创建用户的处理逻辑
    
    user.name = "Alice".to_string();
    user.age = 20;

}