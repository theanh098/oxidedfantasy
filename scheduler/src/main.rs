mod event_status_crawler;
use database::sea_orm::Database;
use dotenv::dotenv;
use scheduler::{CronExpression, Scheduler};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
    let pg_conn = Database::connect(db_url)
        .await
        .expect("fail to connect database");

    Scheduler::new()
        .set_context(pg_conn)
        .add_job(CronExpression::EveryThreeMinutes, &|db| {
            Box::pin(async move {
                event_status_crawler::update_event_status_every_3_mins(&db).await;
            })
        })
        .start()
        .await
        .unwrap_or_else(|err| {
            eprintln!("An error occured: {}", err);
        });
}
