use http::{Response, StatusCode, Method};
use hyper::Body;
use redis::{Client as RedisClient};

use crate::{
    setup::variables::SettingsVars, 
    helpers::{keyval::KeyVal, request::HyperClient, response::{TResult, ApiBody}}, middlewares::request_builder::RequestBuilder
};



pub async fn access_token(hyper_client: HyperClient, redis_client: RedisClient, code: String) -> TResult<ApiBody> {
    let SettingsVars{client_id, redirect_uri, client_secret, ..} = SettingsVars::new();
    let mut con = redis_client.get_async_connection().await.unwrap();


    let auth_code= String::from("");

    let req_body = KeyVal::new().add_list_keyval(vec![
        ("code".into(), code),
        ("grant_type".to_string(), redis::cmd("GET").arg(&["tolumide_test"]).query_async(&mut con).await?),
        ("client_id".to_string(), client_id.clone()),
        ("redirect_uri".to_string(), redirect_uri),
        ("code_verifier".to_string(), "challenge".to_string())
    ]).to_urlencode();

    let content_type = "application/x-www-form-urlencoded;charset=UTF-8";

    let request = RequestBuilder::new(Method::POST, "https://api.twitter.com/2/oauth2/token")
        .with_basic_auth(client_id, client_secret)
        .with_body(req_body, content_type);


    let response_body = Response::builder()
        .status(302).header("Location", "https://api.twitter.com/2/oauth2/token")
        .status(StatusCode::OK).body(Body::from("")).unwrap();

    Ok(response_body)


    

    // let request = RequestBuilder::new(Method::POST, "https://twitter.com/i/oauth2/token");
    
}