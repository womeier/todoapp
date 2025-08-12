use axum::{http::StatusCode, response::IntoResponse, response::Response, Json};
//use tracing::debug;
use tracing::error;

pub enum AppError {
    DbError(diesel::result::Error),
    TaskNotFound,
    BadJson(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DbError(e) => {
                error!("failed to connect to DB {e}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AppError::TaskNotFound => StatusCode::NOT_FOUND.into_response(),
            AppError::BadJson(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
        }
    }
}
