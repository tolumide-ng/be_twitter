use std::time::SystemTime;

use hyper::{Response, StatusCode, Body};
use secrecy::Secret;

use crate::helpers::{hmac_signature::{AuthorizeRequest, ApiCallMethod, Signature}, response::{ApiResponse, ApiResponseBody}};

pub fn authorize_bot() -> ApiResponse {
    let oauth_timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(e) => panic!("SystemTime before UNIX EPOCH!"),
    };


    println!("DURATION>>>>>>>>>>>> {:#?}", oauth_timestamp);

    let app_credentials = AuthorizeRequest {
        include_entities: String::from("true"),
        oauth_consumer_key: String::from("xvz1evFS4wEEPTGEFPHBog"),
        oauth_nonce: String::from("kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg"),
        oauth_signature_method: String::from("HMAC-SHA1"),
        oauth_timestamp,
        oauth_token: Some(Secret::new(String::from("370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb"))),
        oauth_version: String::from("1.0"),
        base_url: String::from("https://api.twitter.com/1.1/statuses/update.json"),
        method: ApiCallMethod::POST,
        consumer_secret: Secret::new("kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw".to_owned()),
        // how to get the user's oauth token - https://developer.twitter.com/en/docs/authentication/oauth-1-0a
        // oauth_token_secret: Some(Secret::new(String::from("LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE")))
    };

    let app_signature = Signature::new(app_credentials);
    
    println!("Hello WORLD!!!!!!!!!!!!!!!!!!!!!!!! {:#?}", app_signature);


    let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    Response::builder()
        .status(StatusCode::OK).body(ok_body)

    // Response::builder().status(StatusCode::OK).body(String::from(""))
    // Ok(Response::default())
    // Response::builder().status(StatusCode::OK).body(String::from(""))
}