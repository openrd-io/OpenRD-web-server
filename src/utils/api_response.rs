use serde::Serialize;

use crate::handlers::error::AppError;
use actix_web::{HttpResponse, Responder, ResponseError};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(messag: &str) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(messag.to_string()),
        }
    }
}

pub struct AppResp<T>(pub T);

impl<T: Serialize> Responder for AppResp<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(ApiResponse::success(self.0))
    }
}