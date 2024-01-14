use database::{repositories::event_status_repository, sea_orm::DatabaseConnection};
use services::fantasy::bootstrap;
use std::error::Error;

pub async fn update_event_status_every_3_mins(
    db: &DatabaseConnection,
) -> Result<(), Box<dyn Error>> {
    let bootstrap = bootstrap::get_bootstrap().await?;

    let _ = event_status_repository::update_events(db, bootstrap.events).await?;

    Ok(())
}
