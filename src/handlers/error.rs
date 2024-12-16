use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use diesel::result::Error as DieselError;
use jsonwebtoken::errors::Error as JwtError;

use crate::{log_error, utils::api_response::ApiResponse};

#[derive(Debug, Display, From)]
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

    #[display("db occur error: {_0}")]
    DBError(DieselError),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => HttpResponse::InternalServerError()
                .json(ApiResponse::<String>::error("Internal Server Error")),
            AppError::NotFound(ref message) => {
                HttpResponse::NotFound().json(ApiResponse::<String>::error(message))
            }
            AppError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(ApiResponse::<String>::error(message))
            }
            AppError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            AppError::JwtError(ref e) => {
                log_error!("JWT error: {}", e);
                HttpResponse::Unauthorized().json(ApiResponse::<String>::error("Unauthorized"))
            }
            AppError::DBError(error) => {
                HttpResponse::InternalServerError()
                    .json(ApiResponse::<String>::error("Internal Server Error"))
            }
        }
    }
}

impl From<DieselError> for AppError {
    fn from(error: DieselError) -> AppError {
        AppError::DBError(error)
    }
}
