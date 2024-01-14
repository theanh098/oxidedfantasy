use crate::error::AppError;
use anyhow::Result;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use database::sea_orm::{Database, DatabaseConnection};
use deadpool_redis::{Config, Runtime};

pub type RedisConnection = deadpool_redis::Connection;

pub struct Redis(pub RedisConnection);
pub struct Postgres(pub DatabaseConnection);

#[derive(Clone)]
pub struct AppState {
    pg_conn: DatabaseConnection,
    redis_pool: deadpool_redis::Pool,
}

#[async_trait]
impl<S> FromRequestParts<S> for Postgres
where
    S: Send + Sync,
    DatabaseConnection: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(DatabaseConnection::from_ref(state)))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Redis
where
    S: Send + Sync,
    deadpool_redis::Pool: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        deadpool_redis::Pool::from_ref(state)
            .get()
            .await
            .map(|conn| Self(conn))
            .map_err(|err| err.into())
    }
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(app_state: &AppState) -> DatabaseConnection {
        app_state.pg_conn.clone()
    }
}

impl FromRef<AppState> for deadpool_redis::Pool {
    fn from_ref(app_state: &AppState) -> deadpool_redis::Pool {
        app_state.redis_pool.clone()
    }
}

impl AppState {
    pub async fn new(db_url: &str) -> Result<Self> {
        Database::connect(db_url)
            .await
            .map_err(|e| e.into())
            .and_then(|pg_conn| {
                Config::from_url("redis://127.0.0.1/")
                    .create_pool(Some(Runtime::Tokio1))
                    .map_err(|e| e.into())
                    .map(|redis_pool| Self {
                        pg_conn,
                        redis_pool,
                    })
            })
    }
}
