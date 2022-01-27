use crate::{helpers::{response::{ApiResponse}}, app::client::AppClient};


pub async fn authorize_bot() -> ApiResponse {
    let app = AppClient::new();
    let response_body = app.oauth2_authorize().await;

    Ok(response_body)
}