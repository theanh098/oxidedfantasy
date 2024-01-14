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
use database::{models::create_user::CreateUser, repositories::user_repository};
use services::google;

use super::shared::generate_tokens;

#[derive(serde::Deserialize)]
pub struct Payload {
    fpl_id: i32,
    access_token: String,
}

pub async fn handler(
    Postgres(db): Postgres,
    Redis(mut redis_conn): Redis,
    Json(payload): Json<Payload>,
) -> Result<Json<AuthenticateResponse>, AppError> {
    let oauth_response = google::authorize(&payload.access_token)
        .await
        .map_err(|err| err.into_app_error())?;

    let existed_user = user_repository::find_first_by_email(&db, &oauth_response.email).await?;

    if existed_user.is_some() {
        return ApiError::ClientError("User already existed".to_string()).into();
    }

    let new_user = user_repository::save(
        &db,
        CreateUser {
            email: &oauth_response.email,
            fpl_id: payload.fpl_id,
            facebook_id: Some(oauth_response.id),
            google_id: None,
        },
    )
    .await?;

    let claims = Claims::new(&new_user, Duration::days(7));
    let sub_claims = SubClaims::new(new_user.id, Duration::days(365));

    let tokens = generate_tokens(&claims, &sub_claims, &mut redis_conn).await?;

    Ok(Json(tokens))
}
