use database::{repositories::event_status_repository, sea_orm::DatabaseConnection};
use services::fantasy::bootstrap;

pub async fn update_event_status_every_3_mins(db: &DatabaseConnection) {
    println!("hello kitty");

    let bootstrap = bootstrap::get_bootstrap().await.unwrap();

    event_status_repository::update_events(db, bootstrap.events)
        .await
        .unwrap();
}
