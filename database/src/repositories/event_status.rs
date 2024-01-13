use sea_orm::{sea_query::OnConflict, DatabaseConnection, EntityTrait, Set};
use services::bootstrap;

use crate::entities::{event_status, prelude::EventStatus};

pub struct EventStatusRepository<'r>(&'r DatabaseConnection);

impl<'r> EventStatusRepository<'r> {
    pub fn new(connection: &'r DatabaseConnection) -> Self {
        Self(connection)
    }

    pub async fn update_events(
        &self,
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
            .exec(self.0)
            .await
            .map(|_| ())
    }
}
