use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum AppError {
    Rejection(RejectedApi),
    Execution(anyhow::Error),
    SurfRequest(services::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Rejection(api_error) => api_error.into_response(),

            AppError::Execution(anyhow_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                to_json(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error occured: {}", anyhow_error),
                ),
            )
                .into_response(),

            AppError::SurfRequest(http_error) => {
                let status_code =
                    StatusCode::from_u16(http_error.status().into()).unwrap_or_default();

                (
                    status_code,
                    to_json(
                        status_code,
                        format!(
                            "Error occured when sending http request, reason: {}",
                            http_error.to_string()
                        ),
                    ),
                )
                    .into_response()
            }
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Execution(err.into())
    }
}

impl<A> From<RejectedApi> for Result<A, AppError> {
    fn from(value: RejectedApi) -> Self {
        Err(AppError::Rejection(value))
    }
}

pub enum RejectedApi {
    AuthenticationError(String),
    ClientError(String),
    InternalError(String),
}

impl IntoResponse for RejectedApi {
    fn into_response(self) -> Response {
        use RejectedApi::*;
        match self {
            AuthenticationError(reason) => (
                StatusCode::UNAUTHORIZED,
                to_json(StatusCode::UNAUTHORIZED, reason),
            )
                .into_response(),

            ClientError(reason) => (
                StatusCode::BAD_REQUEST,
                to_json(StatusCode::BAD_REQUEST, reason),
            )
                .into_response(),

            InternalError(reason) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                to_json(StatusCode::INTERNAL_SERVER_ERROR, reason),
            )
                .into_response(),
        }
    }
}

pub trait FromSurfError {
    fn into_app_error(self) -> AppError;
}

impl FromSurfError for services::Error {
    fn into_app_error(self) -> AppError {
        AppError::SurfRequest(self)
    }
}

pub fn to_json(code: StatusCode, message: String) -> Json<serde_json::Value> {
    Json(json!({
        "code": code.as_u16(),
        "message": message,
        "status": code.to_string()
    }))
}
