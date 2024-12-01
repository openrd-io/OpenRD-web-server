use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: u32,
    biz_id: String,
    name: String,
    email: String,
}

impl User {
    pub fn get_by_id(id: u32) -> User {
        User {
            id: id,
            biz_id: Uuid::new_v4().to_string(),
            name: "John Doe".to_string(),
            email: "test".to_string()
        }   
    }

    pub fn create(user: User) -> User {
        user
    }
}
