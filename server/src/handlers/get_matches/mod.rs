use axum::Json;
use database::{
    entities::sea_orm_active_enums::MatchStatus,
    models::{FindMatchesParams, MatchWithOwnerOpponentAndWinner},
    repositories::match_repository,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::AppError,
    extractors::{security::Guard, state::Postgres, validator::ValidatedQuery},
    responses::PaginationResponse,
};

#[derive(Deserialize)]
enum FindMatchesOption {
    MyMatches,
    MyMatchesAndMyJoinedMatches,
    FlashMatch,
}

#[derive(Deserialize, Validate)]
pub struct QueryParams {
    status: MatchStatus,

    #[validate(range(min = 1))]
    page: u16,

    #[validate(range(min = 1, max = 300))]
    take: u16,

    option: FindMatchesOption,
}

pub async fn handler(
    Postgres(db): Postgres,
    Guard(claims): Guard,
    ValidatedQuery(QueryParams {
        take,
        option,
        status,
        page,
    }): ValidatedQuery<QueryParams>,
) -> Result<Json<PaginationResponse<MatchWithOwnerOpponentAndWinner>>, AppError> {
    let mut find_params = FindMatchesParams::default();

    find_params.page = page;
    find_params.take = take;
    find_params.status = status;

    match option {
        FindMatchesOption::FlashMatch => find_params.status = MatchStatus::Next,
        FindMatchesOption::MyMatches => find_params.created_by = Some(claims.id),
        FindMatchesOption::MyMatchesAndMyJoinedMatches => {
            find_params.joined_by_or_created = Some(claims.id)
        }
    };

    let (matches, total) = match_repository::find_matches(&db, find_params).await?;

    Ok(Json(PaginationResponse {
        nodes: matches,
        page,
        total,
    }))
}
