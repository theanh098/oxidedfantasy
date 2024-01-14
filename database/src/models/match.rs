use crate::entities::{
    match_monitor, match_opponent, r#match, sea_orm_active_enums::MatchStatus, user,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct MatchWithOwnerAndOpponentAndMonitor {
    #[serde(flatten)]
    pub r#match: r#match::Model,
    pub owner: Option<user::Model>,
    pub opponent: Option<match_opponent::Model>,
    pub monitor: Option<match_monitor::Model>,
}

#[derive(Default)]
pub struct FindMatchesParams {
    pub take: u16,
    pub page: u16,
    pub created_by: Option<i32>,
    pub joined_by_or_created: Option<i32>,
    pub status: MatchStatus,
}
