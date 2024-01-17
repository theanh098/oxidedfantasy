use axum::{
    async_trait,
    extract::{
        rejection::{FormRejection, JsonRejection, QueryRejection},
        FromRequest, FromRequestParts, Query, Request,
    },
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Form, Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedQuery<Q>(pub Q);
pub struct ValidatedPayload<P>(pub P);
pub struct ValidatedForm<F>(pub F);

#[async_trait]
impl<S, Q> FromRequestParts<S> for ValidatedQuery<Q>
where
    S: Send + Sync,
    Q: Validate,
    Query<Q>: FromRequestParts<S, Rejection = QueryRejection>,
{
    type Rejection = ExtractError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(query) = Query::<Q>::from_request_parts(parts, state).await?;
        query.validate()?;
        Ok(ValidatedQuery(query))
    }
}

#[async_trait]
impl<S, P> FromRequest<S> for ValidatedPayload<P>
where
    S: Send + Sync,
    P: Validate + DeserializeOwned,
    Json<P>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ExtractError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<P>::from_request(req, state).await?;
        payload.validate()?;
        Ok(ValidatedPayload(payload))
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ExtractError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExtractError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

    #[error(transparent)]
    AxumQueryRejection(#[from] QueryRejection),

    #[error(transparent)]
    AxumPayloadRejection(#[from] JsonRejection),
}

impl IntoResponse for ExtractError {
    fn into_response(self) -> Response {
        match self {
            ExtractError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (
                    StatusCode::BAD_REQUEST,
                    crate::error::to_json(StatusCode::BAD_REQUEST, message),
                )
            }
            _ => (
                StatusCode::BAD_REQUEST,
                crate::error::to_json(StatusCode::BAD_REQUEST, self.to_string()),
            ),
        }
        .into_response()
    }
}
