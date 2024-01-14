use crate::handle_surf_response;

#[derive(serde::Deserialize, Debug)]
pub struct GoogleAuthorizeResponse {
    pub email: String,
    pub id: String,
}

pub async fn authorize(access_token: &str) -> Result<GoogleAuthorizeResponse, surf::Error> {
    let mut response = surf::get("https://www.googleapis.com/oauth2/v1/userinfo")
        .header("Authorization", format!("Bearer {}", access_token))
        .await?;

    handle_surf_response(&mut response).await
}
