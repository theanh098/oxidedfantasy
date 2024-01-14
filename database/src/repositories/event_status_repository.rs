use crate::entities::{event_status, prelude::EventStatus};
use sea_orm::{
    sea_query::OnConflict, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use services::fantasy::bootstrap;

pub async fn update_events(
    db: &DatabaseConnection,
    events: Vec<bootstrap::Event>,
) -> Result<(), sea_orm::error::DbErr> {
    let models: Vec<event_status::ActiveModel> = events
        .into_iter()
        .map(|event| event_status::ActiveModel {
            gameweek: Set(event.id),
            name: Set(event.name),
            average_entry_score: Set(event.average_entry_score),
            data_checked: Set(event.data_checked),
            deadline_time: Set(
                chrono::DateTime::parse_from_rfc3339(&event.deadline_time).unwrap_or_default()
            ),
            finished: Set(event.finished),
            is_current: Set(event.is_current),
            is_next: Set(event.is_next),
            is_previous: Set(event.is_previous),
            deadline_time_epoch: Set(event.deadline_time_epoch),
            highest_scoring_entry: Set(event.highest_scoring_entry.unwrap_or_default()),
            ..Default::default()
        })
        .collect();

    EventStatus::insert_many(models)
        .on_conflict(
            OnConflict::column(event_status::Column::Gameweek)
                .update_columns([
                    event_status::Column::AverageEntryScore,
                    event_status::Column::DataChecked,
                    event_status::Column::DeadlineTime,
                    event_status::Column::DeadlineTimeEpoch,
                    event_status::Column::Finished,
                    event_status::Column::HighestScoringEntry,
                    event_status::Column::IsCurrent,
                    event_status::Column::IsNext,
                    event_status::Column::IsPrevious,
                    event_status::Column::Name,
                ])
                .to_owned(),
        )
        .exec(db)
        .await
        .map(|_| ())
}

pub async fn find_finished_previous_event(
    db: &DatabaseConnection,
) -> Result<Option<event_status::Model>, sea_orm::error::DbErr> {
    EventStatus::find()
        .filter(event_status::Column::IsPrevious.eq(true))
        .filter(event_status::Column::Finished.eq(true))
        .one(db)
        .await
}

pub async fn find_next_event(
    db: &DatabaseConnection,
) -> Result<Option<event_status::Model>, sea_orm::error::DbErr> {
    EventStatus::find()
        .filter(event_status::Column::IsNext.eq(true))
        .one(db)
        .await
}

pub async fn find_current_event(
    db: &DatabaseConnection,
) -> Result<Option<event_status::Model>, sea_orm::error::DbErr> {
    EventStatus::find()
        .filter(event_status::Column::IsCurrent.eq(true))
        .one(db)
        .await
}
