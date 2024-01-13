use surf::StatusCode;

#[derive(serde::Deserialize, Debug)]
pub struct GoogleAuthorizeResponse {
    pub email: String,
    pub id: String,
}

pub async fn authorize(access_token: &str) -> Result<GoogleAuthorizeResponse, surf::Error> {
    let mut response = surf::get("https://www.googleapis.com/oauth2/v1/userinfo")
        .header("Authorization", format!("Bearer {}", access_token))
        .await?;

    let status = response.status();

    if status != StatusCode::Ok {
        let error_data = response
            .body_string()
            .await
            .unwrap_or("Empty error response data".to_owned());

        return Err(surf::Error::from_str(response.status(), error_data));
    }

    response.body_json::<GoogleAuthorizeResponse>().await
}
