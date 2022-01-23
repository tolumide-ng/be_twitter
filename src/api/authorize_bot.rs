use std::time::SystemTime;

use hyper::{Response, StatusCode, Body, Method};
// use http::method::Method;
use secrecy::Secret;
use uuid::Uuid;

use crate::{setup::variables::SettingsVars, helpers::{response::{ApiResponseBody, ApiResponse}, params::KeyPair}, app::client::AppClient};

// use crate::{helpers::response::{ApiResponse, ApiResponseBody}, oauth::OAuthParams}, app::client::AppClient};
// use crate::helpers::oauth::AuthorizeRequest;

pub async fn authorize_bot() -> ApiResponse {

    let SettingsVars{api_key, redirect_uri, api_key_secret, ..} = SettingsVars::new();

    // println!("LEVEL ONE>>>>>>>");

    let ab = AppClient::new();

    // println!("LEVEL TWO {:#?}", ab);
    let consumer = KeyPair::new(api_key, api_key_secret);

    // println!("LEVEL THREE {:#?}", consumer);

    ab.get_request_token(redirect_uri, consumer).await;
    
    let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    // let app_client = AppClient::new(&oauth_signature);
    // println!("-------------------->>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> {:#?}", app_client);
    // let avc = app_client.make_call(&app_credentials).await;

    // println!(":::::::::::::::::::::::::::: {:#?}", avc);

    Response::builder()
        .status(StatusCode::OK).body(ok_body)
}