pub mod facebook;
pub mod fantasy;
pub mod google;
pub use surf::{Error, StatusCode};

async fn handle_surf_response<T: serde::de::DeserializeOwned>(
    response: &mut surf::Response,
) -> Result<T, surf::Error> {
    let status = response.status();

    if status != StatusCode::Ok {
        let error_data = response
            .body_string()
            .await
            .unwrap_or("Empty error response data".to_owned());

        return Err(surf::Error::from_str(response.status(), error_data));
    }

    response.body_json::<T>().await
}
