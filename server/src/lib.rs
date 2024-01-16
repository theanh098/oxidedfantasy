mod error;
mod extractors;
mod handlers;
mod responses;

use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use extractors::state::AppState;
use handlers::{
    create_matches, facebook_register, get_matches, google_register, login, update_fpl_id,
};
use tracing_subscriber::filter::LevelFilter;

#[tokio::main]
pub async fn start() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");

    let app = Router::new()
        .route("/auth/google-register", post(google_register::handler))
        .route("/auth/facebook-register", post(facebook_register::handler))
        .route("/auth/login", post(login::handler))
        .route("/users/update-fpl-id", post(update_fpl_id::handler))
        .route("/users/matches", post(create_matches::handler))
        .route("/users/matches", get(get_matches::handler))
        .with_state(AppState::new(&db_url).await.unwrap());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
