use thiserror::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Not found: {0}")]
    NotFoundError(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    
    #[error("Password hashing error: {0}")]
    PasswordHashError(#[from] bcrypt::BcryptError),
    
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("UUID parse error: {0}")]
    UuidError(#[from] uuid::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::AuthenticationError(_) => (StatusCode::UNAUTHORIZED, "Authentication error"),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            AppError::NotFoundError(_) => (StatusCode::NOT_FOUND, "Not found"),
            AppError::InternalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::PasswordHashError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing error"),
            AppError::JwtError(_) => (StatusCode::UNAUTHORIZED, "JWT error"),
            AppError::UuidError(_) => (StatusCode::BAD_REQUEST, "Invalid UUID"),
        };
        
        let body = json!({
            "error": message,
            "status": status.as_u16(),
        });
        
        (status, axum::Json(body)).into_response()
    }
}