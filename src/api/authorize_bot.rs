use std::time::SystemTime;

use hyper::{Response, StatusCode, Body, Method};
// use http::method::Method;
use secrecy::Secret;
use uuid::Uuid;

use crate::{helpers::{hmac_signature::{Signature}, 
    response::{ApiResponse, ApiResponseBody}}, app::client::AppClient
};
use crate::helpers::app_credentials::AuthorizeRequest;

pub async fn authorize_bot() -> ApiResponse {
    let oauth_timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(e) => panic!("SystemTime before UNIX EPOCH!"),
    };


    println!("DURATION>>>>>>>>>>>> {:#?}", oauth_timestamp);

    let app_credentials = AuthorizeRequest {
        include_entities: String::from("true"),
        // oauth_consumer_key: String::from("xvz1evFS4wEEPTGEFPHBog"),
        oauth_nonce: base64::encode(Uuid::new_v4().to_string()),
        oauth_signature_method: String::from("HMAC-SHA1"),
        oauth_timestamp,
        oauth_token: None,
        oauth_version: String::from("1.0"),
        base_url: String::from("https://api.twitter.com/oauth/request_token"),
        method: Method::POST.to_string(),
        // how to get the user's oauth token - https://developer.twitter.com/en/docs/authentication/oauth-1-0a
        // oauth_token_secret: Some(Secret::new(String::from("LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE")))
    };

    let oauth_signature = Signature::new(&app_credentials);
    
    println!("Hello WORLD!!!!!!!!!!!!!!!!!!!!!!!! {:#?}", oauth_signature);


    let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    let app_client = AppClient::new(&oauth_signature);
    println!("-------------------->>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> {:#?}", app_client);
    let avc = app_client.make_call(&app_credentials).await;

    println!(":::::::::::::::::::::::::::: {:#?}", avc);

    Response::builder()
        .status(StatusCode::OK).body(ok_body)
}