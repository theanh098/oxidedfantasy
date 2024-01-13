use deadpool_redis::redis::cmd;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::env;

use crate::{
    extractors::{
        security::{Claims, SubClaims},
        state::RedisConnection,
    },
    responses::auth::AuthenticateResponse,
};

pub async fn generate_tokens(
    claims: &Claims,
    sub_claims: &SubClaims,
    redis_conn: &mut RedisConnection,
) -> anyhow::Result<AuthenticateResponse> {
    let secret = env::var("JWT_SECRET")?;
    let refresh_secret = env::var("JWT_REFRESH_SECRET")?;

    let header = Header::new(Algorithm::HS256);

    let secret_key = EncodingKey::from_secret(secret.as_bytes());
    let refresh_key = EncodingKey::from_secret(refresh_secret.as_bytes());

    let access_token = encode(&header, claims, &secret_key)?;
    let renew_token = encode(&header, sub_claims, &refresh_key)?;

    cmd("SET")
        .arg(renew_key(claims.id))
        .arg(&renew_token)
        .query_async(redis_conn)
        .await?;

    Ok(AuthenticateResponse {
        access_token,
        renew_token,
    })
}

fn renew_key(id: i32) -> String {
    format!("renew_token_of_{id}")
}
