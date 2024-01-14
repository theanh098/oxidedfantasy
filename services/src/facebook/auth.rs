use crate::handle_surf_response;

#[derive(serde::Deserialize, Debug)]
pub struct FacebookAuthorizeResponse {
    pub email: String,
    pub id: String,
}

pub async fn authorize(access_token: &str) -> Result<FacebookAuthorizeResponse, surf::Error> {
    let mut response = surf::get(format!(
        "https://graph.facebook.com/v12.0/me?fields=id,email&access_token=${}",
        access_token
    ))
    .await?;

    handle_surf_response(&mut response).await
}
