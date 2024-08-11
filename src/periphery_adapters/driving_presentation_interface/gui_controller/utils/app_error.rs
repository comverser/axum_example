use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub struct AppError {
    status_code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(status_code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status_code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(ResponseMessage {
                message: self.message,
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}
