use crate::{
    error::AppError,
    extractors::{security::Guard, state::Postgres, validator::ValidatedPayload},
};
use database::repositories::user_repository;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(range(min = 1))]
    fpl_id: i32,
}

pub async fn handler(
    Postgres(db): Postgres,
    Guard(claims): Guard,
    ValidatedPayload(payload): ValidatedPayload<Payload>,
) -> Result<(), AppError> {
    let _ = user_repository::update_fpl_id(&db, claims.id, payload.fpl_id).await?;

    Ok(())
}
