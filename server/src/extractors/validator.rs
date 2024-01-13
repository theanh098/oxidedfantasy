use crate::error::ApiError;
use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Query, Request},
    http::request::Parts,
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedQuery<Q>(pub Q);
pub struct ValidatedPayload<P>(pub P);

#[async_trait]
impl<S, Q> FromRequestParts<S> for ValidatedQuery<Q>
where
    S: Send + Sync,
    Q: Validate,
    Query<Q>: FromRequestParts<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let query = Query::<Q>::from_request_parts(parts, state).await;

        if let Ok(Query(data)) = query {
            match data.validate() {
                Ok(_) => Ok(ValidatedQuery(data)),
                Err(err) => Err(ApiError::ClientError(err.to_string())),
            }
        } else {
            Err(ApiError::ClientError("Invalid query string".into()))
        }
    }
}

#[async_trait]
impl<S, P> FromRequest<S> for ValidatedPayload<P>
where
    S: Send + Sync,
    P: Validate + DeserializeOwned,
    Json<P>: FromRequest<S>,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let json = Json::<P>::from_request(req, state).await;

        if let Ok(Json(json_body)) = json {
            match json_body.validate() {
                Ok(_) => Ok(ValidatedPayload(json_body)),
                Err(err) => Err(ApiError::ClientError(err.to_string())),
            }
        } else {
            Err(ApiError::ClientError("Invalid json body".into()))
        }
    }
}
