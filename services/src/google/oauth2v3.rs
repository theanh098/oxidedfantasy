#[derive(serde::Deserialize, Debug)]
pub struct GoogleAuthorizeResponse {
    pub email: String,
    pub sub: String,
}

pub async fn authorize(access_token: &str) -> Result<GoogleAuthorizeResponse, surf::Error> {
    surf::get(format!(
        "https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={access_token}",
    ))
    .await?
    .body_json::<GoogleAuthorizeResponse>()
    .await
}
