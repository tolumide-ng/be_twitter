use http::{Response, StatusCode, Method};
use hyper::Body;
use redis::{Client as RedisClient};

use crate::{
    setup::variables::SettingsVars, 
    helpers::{keyval::KeyVal, request::HyperClient, response::{TResult, ApiBody, make_request, ApiResponseBody}}, middlewares::request_builder::RequestBuilder
};



pub async fn access_token(hyper_client: HyperClient, redis_client: RedisClient, auth_code: String) -> TResult<ApiBody> {
    println!("\n\n WITHIN THE ACCESS TOKEN {:#?} \n\n", auth_code);

    let SettingsVars{client_id, redirect_uri, client_secret, ..} = SettingsVars::new();
    let mut con = redis_client.get_async_connection().await.unwrap();


    let req_body = KeyVal::new().add_list_keyval(vec![
        ("code".into(), auth_code.clone()), //
        ("grant_type".to_string(), auth_code), //
        ("client_id".to_string(), client_id.clone()), //
        ("redirect_uri".to_string(), redirect_uri), //
        ("code_verifier".to_string(), redis::cmd("GET").arg(&["tolumide_test"]).query_async(&mut con).await?) //
    ]).to_urlencode();

    println!("THE REQUEST BODY FROM HERE {:#?}", &req_body);

    let content_type = "application/x-www-form-urlencoded";

    let request = RequestBuilder::new(Method::POST, "https://api.twitter.com/2/oauth2/token".into())
        .with_basic_auth(client_id, client_secret)
        .with_body(req_body, content_type).build_request();


    println!("\n\n ***********\n WHAT THE REQUEST SHOULD LOOK LIKE {:#?} \n***************\n\n", request);

    // let response_body = Response::builder().status(302)

    let response = make_request(request, hyper_client.clone()).await?;

    println!("|||||||||||||| THE RESPONSE |||||||||||||| {:#?}", response);


    let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    let response_body = Response::builder()
        .status(StatusCode::OK).body(ok_body).unwrap();

    Ok(response_body)

    

    // let request = RequestBuilder::new(Method::POST, "https://twitter.com/i/oauth2/token");
    
}