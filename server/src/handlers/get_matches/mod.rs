use crate::{
    error::AppError,
    extractors::{security::Guard, state::Postgres, validator::ValidatedQuery},
    responses::PaginationResponse,
};
use axum::Json;
use database::{
    entities::sea_orm_active_enums::MatchStatus,
    models::{FindMatchesParams, MatchWithOwnerOpponentAndWinner},
    repositories::match_repository,
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

static SEASON_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\d{4}-\d{4}$"#).unwrap());

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
    page: u64,

    #[validate(range(min = 1, max = 300))]
    take: u64,

    option: FindMatchesOption,

    #[validate(regex = "SEASON_PATTERN")]
    season: Option<String>,
}

pub async fn handler(
    Postgres(db): Postgres,
    Guard(claims): Guard,
    ValidatedQuery(QueryParams {
        take,
        option,
        status,
        page,
        season,
    }): ValidatedQuery<QueryParams>,
) -> Result<Json<PaginationResponse<MatchWithOwnerOpponentAndWinner>>, AppError> {
    let mut find_params = FindMatchesParams::default();

    find_params.page = page;
    find_params.take = take;
    find_params.season = season;
    find_params.status = status;

    match option {
        FindMatchesOption::FlashMatch => {
            find_params.status = MatchStatus::Next;
            find_params.exclude_created_by = Some(claims.id);
        }
        FindMatchesOption::MyMatches => {
            find_params.created_by = Some(claims.id);
        }
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
