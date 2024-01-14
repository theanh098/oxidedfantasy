//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use super::sea_orm_active_enums::ChipRule;
use super::sea_orm_active_enums::MatchStatus;
use super::sea_orm_active_enums::TransferRule;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "match")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub season: String,
    pub created_date: DateTimeWithTimeZone,
    pub matched_at: Option<DateTimeWithTimeZone>,
    pub bet_amount: i32,
    pub owner_id: i32,
    pub opponent_id: Option<i32>,
    pub is_draw: bool,
    pub is_matched: bool,
    #[sea_orm(column_type = "JsonBinary")]
    pub metadata: Json,
    pub opponent_point: i32,
    pub owner_point: i32,
    pub winner_id: Option<i32>,
    pub gameweek: i32,
    pub transfer_rule: TransferRule,
    pub chip_rule: ChipRule,
    pub status: MatchStatus,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OpponentId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    User3,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    User2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::WinnerId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    User1,
}

impl ActiveModelBehavior for ActiveModel {}
