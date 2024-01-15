use crate::{
    error::{ApiError, AppError},
    extractors::{security::Guard, state::Postgres, validator::ValidatedPayload},
};
use database::{
    entities::{
        r#match,
        sea_orm_active_enums::{ChipRule, TransferRule},
    },
    repositories::{event_status_repository, match_repository, user_repository},
    sea_orm::Set,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Clone)]
pub struct Payload {
    #[validate(range(min = 1))]
    bet: i32,

    #[validate(range(min = 1))]
    quantity: u32,

    chip_rule: ChipRule,

    transfer_rule: TransferRule,

    is_private: Option<bool>,

    #[validate(range(min = 1))]
    min_week_started: Option<i32>,
}

pub async fn handler(
    Postgres(db): Postgres,
    Guard(claims): Guard,
    ValidatedPayload(payload): ValidatedPayload<Payload>,
) -> Result<(), AppError> {
    let next_event = event_status_repository::find_next_event(&db).await?;

    let Some(next_event) = next_event else {
        return ApiError::InternalError("not found next gameweek".to_owned()).into();
    };

    let total_d_coin = payload.quantity as i32 * payload.bet;

    {
        let user = user_repository::find_by_id(&db, claims.id).await?;

        let Some(user) = user else {
            return ApiError::AuthenticationError("not found user".to_owned()).into();
        };

        if user.d_coin < total_d_coin {
            return ApiError::ClientError("not d_coin enough".to_owned()).into();
        }
    }

    let matches_vec = vec![1; payload.quantity as usize];

    let matches = matches_vec
        .into_iter()
        .map(|_| r#match::ActiveModel {
            bet_amount: Set(payload.bet),
            chip_rule: Set(payload.chip_rule.clone()),
            transfer_rule: Set(payload.transfer_rule.clone()),
            gameweek: Set(next_event.gameweek),
            owner_id: Set(claims.id),
            season: Set("23-24".to_owned()),
            is_private: Set(payload.is_private.unwrap_or_default()),
            ..Default::default()
        })
        .collect::<Vec<r#match::ActiveModel>>();

    match_repository::create_matches(&db, claims.id, matches, next_event.gameweek, total_d_coin)
        .await?;

    Ok(())
}
