use database::sea_orm::Database;
use dotenv::dotenv;
use std::collections::HashMap;
use watcher::start_listening;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
    let db = Database::connect(&db_url).await.unwrap();
    let pool = db.get_postgres_connection_pool();

    let mut workers = HashMap::new();

    workers.insert(
        "something_change",
        |payload: serde_json::Value, _db| async move {
            println!("json row payload: {}", payload);
            Ok(())
        },
    );

    start_listening(pool, &db, workers)
        .await
        .unwrap_or_else(|err| {
            eprintln!("An error occured from watcher: {}", err);
        });

    Ok(())
}
