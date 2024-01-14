mod error;
mod extractors;
mod handlers;
mod responses;

use axum::{routing::post, Router};
use dotenv::dotenv;
use extractors::state::AppState;
use handlers::{facebook_register, google_register};

#[tokio::main]
pub async fn start() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");

    let app = Router::new()
        .route("/auth/google-register", post(google_register::handler))
        .route("/auth/facebook-register", post(facebook_register::handler))
        .with_state(AppState::new(&db_url).await.unwrap());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
