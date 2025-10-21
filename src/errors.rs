use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BaseError {
    #[error("{0}")]
    Forbidden(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InternalServerError(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    NoContent(String),

    #[error("{0}")]
    Conflict(String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

impl ResponseError for BaseError {
    fn error_response(&self) -> HttpResponse {
        tracing::error!("Error: {}", self);

        match self {
            BaseError::Forbidden(msg) => HttpResponse::Forbidden().json(msg),
            BaseError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            BaseError::InternalServerError(msg) => HttpResponse::InternalServerError().json(msg),
            BaseError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            BaseError::NoContent(msg) => HttpResponse::NoContent().json(msg),
            BaseError::Conflict(msg) => HttpResponse::Conflict().json(msg),
            BaseError::SqlxError(e) => HttpResponse::InternalServerError().json(e.to_string()),
        }
    }
}

impl Drop for BaseError {
    fn drop(&mut self) {
        tracing::error!("{:?}", self.to_string());
    }
}
