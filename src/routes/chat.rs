use crate::handlers::auth::AuthenticatedUser;
use crate::handlers::db::DbPool;
use crate::handlers::error::AppError;
use crate::models::chat::{ChatGroup, ChatGroupDTO, ChatMessage, ChatMessageDTO};
use crate::utils::api_response::ApiResponse;
use actix_web::{get, post, web, HttpResponse};
use actix_web_grants::protect;
use diesel::dsl::count;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListMessagesQuery {
    pub page: i64,
    pub per_page: i64,
}

#[get("/chat/groups")]
#[protect("USER")]
async fn query_chat_group_list(
    pool: web::Data<DbPool>,
    page: web::Query<ListMessagesQuery>,
    user: AuthenticatedUser, // 使用提取器获取用户信息
) -> Result<HttpResponse, AppError> {
    let user_id = user.0.sub.clone();

    let resp: Result<(Vec<ChatGroup>, i64), AppError> = web::block(move || {
        let mut conn = pool.get().map_err(|_| AppError::InternalServerError)?;

        let groups =
            ChatGroup::find_by_user_id(&mut conn, user_id.to_string(), page.page, page.per_page)?;

        let count = ChatGroup::count_by_user(&mut conn, user_id.to_string())?;

        Ok((groups, count))
    })
    .await
    .map_err(|_| AppError::InternalServerError)?;

    let (groups, count) = resp.map_err(AppError::from)?;

    let resp = serde_json::json!({
        "content": groups,
        "total": count
    });

    Ok(HttpResponse::Ok().json(ApiResponse::success(resp)))
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
    let response_data = serde_json::json!({
        "group_id": group.biz_id.to_string(),
        "description": group.description,
        "default_message": ChatMessage::default_messages()
    });
    Ok(HttpResponse::Created().json(ApiResponse::success(response_data)))
}

#[post("/chat/groups/{id}/messages")]
#[protect("USER")]
async fn create_message(
    pool: web::Data<DbPool>,
    group_id: web::Path<String>,
    message_dto: web::Json<ChatMessageDTO>,
) -> Result<HttpResponse, AppError> {
    let group_id = group_id.into_inner();
    let mut conn = pool.get().map_err(|_| AppError::InternalServerError)?;

    let mut new_message = message_dto.into_inner();
    new_message.group_id = Some(group_id);
    log::info!("new_message: {:?}", new_message);
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
    group_id: web::Path<String>,
    query: web::Query<ListMessagesQuery>,
) -> Result<HttpResponse, AppError> {
    // 实现列出消息逻辑
    let group_id = group_id.into_inner();

    let resp: Result<(Vec<ChatMessage>, i64), AppError> = web::block(move || {
        let mut conn = pool.get().map_err(|_| AppError::InternalServerError)?;

        let messages =
            ChatMessage::find_by_group_id(&mut conn, group_id.clone(), query.page, query.per_page)?;

        let count = ChatMessage::count_by_group(&mut conn, group_id)?;

        Ok((messages, count))
    })
    .await
    .map_err(|_| AppError::InternalServerError)?;

    let (messages, count) = resp.map_err(AppError::from)?;

    let resp = serde_json::json!({
        "content": messages,
        "total": count
    });

    Ok(HttpResponse::Ok().json(resp))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_chat_group)
        .service(create_message)
        .service(query_chat_group_list)
        .service(list_messages);
}
