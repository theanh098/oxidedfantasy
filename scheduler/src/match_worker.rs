use std::error::Error;

use database::{
    repositories::{event_status_repository, match_repository},
    sea_orm::DatabaseConnection,
};

pub async fn update_matches_to_live(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    let current_event = event_status_repository::find_current_event(db).await?;

    let Some(current_event) = current_event else {
        return Ok(());
    };

    let _ = match_repository::update_all_next_round_to_live_by_gameweek(db, current_event.gameweek)
        .await?;

    Ok(())
}

pub async fn update_matches_to_finished(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    let previous_event = event_status_repository::find_finished_previous_event(db).await?;

    let Some(previous_event) = previous_event else {
        return Ok(());
    };

    let _ = match_repository::update_all_live_to_finished_by_gameweek(db, previous_event.gameweek)
        .await?;

    Ok(())
}
