use crate::entities::sea_orm_active_enums::MatchStatus;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct MatchWithOwnerOpponentAndWinner {
    #[serde(flatten)]
    r#match: PartialMatch,

    owner: PlayerOnMatch,

    #[serde(skip_serializing_if = "Option::is_none")]
    opponent: Option<PlayerOnMatch>,

    #[serde(skip_serializing_if = "Option::is_none")]
    winner: Option<PlayerOnMatch>,
}
#[derive(Serialize)]

struct PartialMatch {
    id: i32,
    gameweek: i32,
    status: MatchStatus,
}
#[derive(Serialize)]
struct PlayerOnMatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fpl_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
}

impl FromQueryResult for MatchWithOwnerOpponentAndWinner {
    fn from_query_result(
        res: &sea_orm::prelude::QueryResult,
        _pre: &str,
    ) -> Result<Self, sea_orm::prelude::DbErr> {
        let r#match = PartialMatch {
            id: res.try_get("", "id")?,
            gameweek: res.try_get("", "gameweek")?,
            status: res.try_get("", "status")?,
        };

        let owner = PlayerOnMatch {
            id: res.try_get("owner_", "id")?,
            first_name: res.try_get("owner_", "player_first_name")?,
            last_name: res.try_get("owner_", "player_last_name")?,
            name: res.try_get("owner_", "name")?,
            fpl_id: res.try_get("owner_", "fpl_id")?,
        };

        let opponent = match res.try_get("opponent_", "id")? {
            Some(id) => Some(PlayerOnMatch {
                id,
                first_name: res.try_get("opponent_", "player_first_name")?,
                last_name: res.try_get("opponent_", "player_last_name")?,
                name: res.try_get("opponent_", "name")?,
                fpl_id: res.try_get("opponent_", "fpl_id")?,
            }),
            None => None,
        };

        let winner = match res.try_get("opponent_", "id")? {
            Some(id) => Some(PlayerOnMatch {
                id,
                first_name: res.try_get("winner_", "first_name")?,
                last_name: res.try_get("winner_", "last_name")?,
                name: res.try_get("winner_", "name")?,
                fpl_id: res.try_get("winner_", "fpl_id")?,
            }),
            None => None,
        };

        Ok(Self {
            r#match,
            opponent,
            owner,
            winner,
        })
    }
}

#[derive(Default)]
pub struct FindMatchesParams {
    pub take: u64,
    pub page: u64,
    pub created_by: Option<i32>,
    pub joined_by_or_created: Option<i32>,
    pub exclude_created_by: Option<i32>,
    pub status: MatchStatus,
    pub season: Option<String>,
}
