use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("font not found")]
    FontNotFound,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ImageError(#[from] image::ImageError),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    #[error("Timeout")]
    Timeout,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            AppError::Unknown(_) => {
                let message = format!("Unknown Error: [{}]", self).replace('\n', ", ");
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::ReqwestError(_) => {
                let message = format!("Reqwest Error: [{}]", self).replace('\n', ", ");
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::SerdeJsonError(_) => {
                let message = format!("Serde JSON Error: [{}]", self).replace('\n', ", ");
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::Timeout => {
                let message = format!("Timeout Error: [{}]", self).replace('\n', ", ");
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::ImageError(_) => {
                let message = format!("Image Error: [{}]", self).replace('\n', ", ");
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::StdIoError(_) => {
                let message = format!("Std IO Error: [{}]", self).replace('\n', ", ");
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::FontNotFound => {
                let message = format!("Font not found: [{}]", self).replace('\n', ", ");
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
        }
        .into_response()
    }
}
