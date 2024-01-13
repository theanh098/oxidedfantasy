#[derive(serde::Serialize)]
pub struct AuthenticateResponse {
    pub access_token: String,
    pub renew_token: String,
}
