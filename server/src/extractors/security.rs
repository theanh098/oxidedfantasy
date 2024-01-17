use crate::error::RejectedApi;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::Utc;
use database::entities::user::Model as User;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub exp: u32,
    pub id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct SubClaims {
    pub exp: u32,
    pub sub: i32,
}

pub struct Guard(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for Guard
where
    S: Send + Sync,
{
    type Rejection = RejectedApi;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let access_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

        let bearer = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| RejectedApi::AuthenticationError("Missing Authorization".into()))?;

        jsonwebtoken::decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(access_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|err| match err.kind() {
            ErrorKind::ExpiredSignature => RejectedApi::AuthenticationError("Expired token".into()),
            _ => RejectedApi::AuthenticationError("Invalid token".into()),
        })
        .map(|token_data| Self(token_data.claims))
    }
}

impl Claims {
    pub fn new(user: &User, expired: chrono::Duration) -> Self {
        Self {
            id: user.id,
            exp: Utc::now().checked_add_signed(expired).unwrap().timestamp() as u32,
        }
    }
}

impl SubClaims {
    pub fn new(sub: i32, expired: chrono::Duration) -> Self {
        Self {
            sub,
            exp: Utc::now().checked_add_signed(expired).unwrap().timestamp() as u32,
        }
    }
}
