use crate::{
    error::{ApiError, AppError, FromSurfError},
    extractors::{
        security::{Claims, SubClaims},
        state::{Postgres, Redis},
    },
    responses::auth::AuthenticateResponse,
};
use axum::Json;
use chrono::Duration;
use database::repositories::user_repository;
use serde::Deserialize;
use services::{facebook, google};

use super::shared::generate_tokens;

#[derive(Deserialize)]
pub enum LoginOption {
    Facebook,
    Google,
}

#[derive(Deserialize)]
pub struct Payload {
    access_token: String,
    option: LoginOption,
}

pub async fn handler(
    Postgres(db): Postgres,
    Redis(mut redis_conn): Redis,
    Json(payload): Json<Payload>,
) -> Result<Json<AuthenticateResponse>, AppError> {
    let (google_id, facebook_id) = match payload.option {
        LoginOption::Google => google::authorize(&payload.access_token)
            .await
            .map(|res| (Some(res.id), None))
            .map_err(|err| err.into_app_error())?,

        LoginOption::Facebook => facebook::authorize(&payload.access_token)
            .await
            .map(|res| (None, Some(res.id)))
            .map_err(|err| err.into_app_error())?,
    };

    let user = user_repository::find_first_by_platform_id(&db, google_id, facebook_id).await?;

    let Some(user) = user else {
        return ApiError::AuthenticationError("user not found".to_owned()).into();
    };

    let claims = Claims::new(&user, Duration::days(7));
    let sub_claims = SubClaims::new(user.id, Duration::days(365));

    let tokens = generate_tokens(&claims, &sub_claims, &mut redis_conn).await?;

    Ok(Json(tokens))
}
