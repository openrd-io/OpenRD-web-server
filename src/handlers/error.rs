use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use diesel::result::Error as DieselError;
use jsonwebtoken::errors::Error as JwtError;

use crate::app_error;


#[derive(Debug, Display,From)]
pub enum AppError {
    #[display("Internal Server Error")]
    InternalServerError,

    #[display("NotFound: {_0}")]
    NotFound(String),

    #[display("BadRequest: {_0}")]
    BadRequest(String),

    #[display("Unauthorized")]
    Unauthorized,

    #[from]
    JwtError(JwtError),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => HttpResponse::InternalServerError().json("Internal Server Error"),
            AppError::NotFound(ref message) => HttpResponse::NotFound().json(message),
            AppError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            AppError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            AppError::JwtError(ref e) => {
                app_error!("JWT error: {}", e);
                HttpResponse::Unauthorized().json("Unauthorized")
            }
        }
    }
}

impl From<DieselError> for AppError {
    fn from(error: DieselError) -> AppError {
        match error {
            DieselError::NotFound => AppError::NotFound("Record not found".into()),
            _ => AppError::InternalServerError,
        }
    }
} 