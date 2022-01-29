use std::collections::HashMap;

use hyper::{Body, Response, StatusCode, Request};
use redis::{Client as RedisClient};
use crate::{helpers::{response::{ApiResponse, ApiResponseBody, ApiBody}, request::{get_param_from_uri, HyperClient}, keyval::KeyVal}, setup::variables::SettingsVars};


// struct UriParams(String);

// impl UriParams {
//     pub fn new() -> Self {}
// }

// todo() - I should move all the controllers used to handle 2.0 authentication into one struct and represent them as methods within the struct
pub async fn handle_redirect(req: Request<hyper::Body>, hyper_client: &HyperClient, redis_client: RedisClient) -> ApiResponse<ApiBody> {
    let SettingsVars{state, ..} = SettingsVars::new();

    let query_params = get_param_from_uri(req.uri());
    // let act = KeyVal::to_access_token(query_params.unwrap());
    if let Some(dict) = query_params {
        //todo() find a way to confirm if code and state are present in the dic, handle the edge cases
        let obtained_state = dict.get("state").unwrap();
        if *obtained_state != state {
            panic!("Please try again later, state isn't same, CSRF?")
        }
        
        let auth_code = dict.get("code").unwrap();
    };

     let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    let response_body = Response::builder()
        .status(StatusCode::OK).body(ok_body).unwrap();

    Ok(response_body)
}