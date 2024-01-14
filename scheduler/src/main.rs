mod event_status_crawler;
mod match_worker;

use database::sea_orm::{ConnectOptions, Database};
use dotenv::dotenv;
use scheduler::{CronExpression, Scheduler};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
    let pg_conn = Database::connect(ConnectOptions::new(db_url))
        .await
        .expect("fail to connect database");

    Scheduler::new()
        .set_context(pg_conn)
        .add_job(CronExpression::EveryThreeMinutes, &|db| {
            Box::pin(async move {
                event_status_crawler::update_event_status_every_3_mins(&db)
                    .await
                    .unwrap_or_else(|err| {
                        eprintln!("An error occured when crawl event status: {}", err);
                    });
            })
        })
        .add_job(CronExpression::EveryFiveMinutes, &|db| {
            Box::pin(async move {
                match_worker::update_matches_to_live(&db)
                    .await
                    .unwrap_or_else(|err| {
                        eprintln!("An error occured when update matches to live: {}", err);
                    });
            })
        })
        .add_job(CronExpression::EveryFiveMinutes, &|db| {
            Box::pin(async move {
                match_worker::update_matches_to_finished(&db)
                    .await
                    .unwrap_or_else(|err| {
                        eprintln!("An error occured when update matches to finished: {}", err);
                    });
            })
        })
        .start()
        .await
        .unwrap_or_else(|err| {
            eprintln!("An error occured: {}", err);
        });
}
