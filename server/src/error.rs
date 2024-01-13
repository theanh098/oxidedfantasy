use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum AppError {
    ExpectedError(ApiError),
    UnExpectedError(anyhow::Error),
    HttpSurfError(services::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ExpectedError(api_error) => api_error.into_response(),

            AppError::UnExpectedError(anyhow_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error occured: {}", anyhow_error),
            )
                .into_response(),

            AppError::HttpSurfError(http_error) => {
                println!("http_error: {:#?}", http_error);

                return  (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "cause": "not",
                        "message": format!("Error occured when sending http request, reason: {}",http_error.to_string())
                    })),
                ).into_response();
            }
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::UnExpectedError(err.into())
    }
}

impl<A> From<ApiError> for Result<A, AppError> {
    fn from(value: ApiError) -> Self {
        Err(AppError::ExpectedError(value))
    }
}

pub enum ApiError {
    AuthenticationError(String),
    ClientError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use ApiError::*;
        match self {
            AuthenticationError(reason) => (StatusCode::UNAUTHORIZED, reason).into_response(),
            ClientError(reason) => (StatusCode::BAD_REQUEST, reason).into_response(),
        }
    }
}

pub trait FromSurfError {
    fn into_app_error(self) -> AppError;
}

impl FromSurfError for services::Error {
    fn into_app_error(self) -> AppError {
        AppError::HttpSurfError(self)
    }
}
