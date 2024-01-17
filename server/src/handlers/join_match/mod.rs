use crate::{
    error::{AppError, RejectedApi},
    extractors::{security::Guard, state::Postgres},
};
use axum::extract::Path;
use database::{
    entities::sea_orm_active_enums::MatchStatus,
    repositories::{match_repository, user_repository},
};

pub async fn handler(
    Postgres(db): Postgres,
    Guard(claims): Guard,
    Path(match_id): Path<i32>,
) -> Result<(), AppError> {
    let r#match = match_repository::find_by_id(&db, match_id).await?;
    let user = user_repository::find_by_id(&db, claims.id).await?;

    let Some(r#match) = r#match else {
        return RejectedApi::ClientError("not found match".to_owned()).into();
    };

    let Some(user) = user else {
        return RejectedApi::AuthenticationError("not found user".to_owned()).into();
    };

    if r#match.status != MatchStatus::Next {
        return RejectedApi::ClientError("the match is not for next round".to_owned()).into();
    }

    if r#match.opponent_id.is_some() {
        return RejectedApi::ClientError("the match is full".to_owned()).into();
    }

    if r#match.owner_id == claims.id {
        return RejectedApi::ClientError("can not join the own match".to_owned()).into();
    }

    if user.d_coin < r#match.bet_amount {
        return RejectedApi::ClientError("not d_coin enough".to_owned()).into();
    }

    match_repository::update_when_user_join_match(&db, match_id, claims.id).await?;

    Ok(())
}
