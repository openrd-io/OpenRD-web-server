use actix_web::{get, post, delete, web, HttpResponse};
use actix_web_grants::protect;
use crate::handlers::error::AppError;
use crate::models::chat::{ChatGroup, ChatGroupDTO, ChatMessage, ChatMessageDTO};
use crate::handlers::db::DbPool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListMessagesQuery {
    pub page: i64,
    pub per_page: i64,
}

#[post("/chat/groups")]
#[protect("USER")]
async fn create_chat_group(
    pool: web::Data<DbPool>,
    group_dto: web::Json<ChatGroupDTO>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    
    let group = web::block(move || ChatGroup::create(&mut conn, &group_dto))
        .await
        .map_err(|_| AppError::InternalServerError)?
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(group))
}

#[post("/chat/groups/{id}/messages")]
#[protect("USER")]
async fn create_message(
    pool: web::Data<DbPool>,
    group_id: web::Path<i32>,
    message_dto: web::Json<ChatMessageDTO>,
) -> Result<HttpResponse, AppError> {
    let group_id = group_id.into_inner();
    let mut conn = pool.get().map_err(|_| AppError::InternalServerError)?;

    let mut new_message = message_dto.into_inner();
    new_message.group_id = group_id;

    let message = web::block(move || ChatMessage::create(&mut conn, &new_message))
        .await
        .map_err(|_| AppError::InternalServerError)?
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(message))
}


#[get("/chat/groups/{id}/messages")]
#[protect("USER")]
async fn list_messages(
    pool: web::Data<DbPool>,
    group_id: web::Path<i32>,
    query: web::Query<ListMessagesQuery>,
) -> Result<HttpResponse, AppError> {
    // 实现列出消息逻辑
    let group_id = group_id.into_inner();
    let mut conn = pool.get().map_err(|_| AppError::InternalServerError)?;

    let messages = web::block(move || ChatMessage::find_by_group_id(&mut conn, group_id, query.page, query.per_page))
        .await
        .map_err(|_| AppError::InternalServerError)?
        .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(messages))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_chat_group)
       .service(create_message)
       .service(list_messages);
} 