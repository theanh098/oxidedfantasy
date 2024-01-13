use database::{repositories::event_status::EventStatusRepository, sea_orm::DatabaseConnection};
use services::bootstrap;

pub async fn update_event_status_every_3_mins(db: &DatabaseConnection) {
    println!("hello kitty");

    let bootstrap = bootstrap::get_bootstrap().await.unwrap();

    EventStatusRepository::new(db)
        .update_events(bootstrap.events)
        .await
        .unwrap();
}
