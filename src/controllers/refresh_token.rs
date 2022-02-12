use std::collections::HashMap;

use http::{StatusCode};
use hyper::{Request, Method};
use redis::{Client as RedisClient};
use serde_json::Value;

use crate::{helpers::{request::HyperClient, keyval::KeyVal, response::{make_request, TResult, ApiBody, ResponseBuilder}}, setup::variables::SettingsVars, middlewares::request_builder::RequestBuilder, errors::response::TError};

pub async fn refresh_token(_req: Request<hyper::Body>, hyper_client: HyperClient, redis_client: RedisClient) -> TResult<ApiBody> {
    let SettingsVars {client_id, client_secret, twitter_v2, ..} = SettingsVars::new();

    let mut con = redis_client.get_async_connection().await.unwrap();
    let content = "application/x-www-form-urlencoded";

    println!("LEVEL TWO");

    // todo()! - Make the Grant_type an enum with From method to convert into string - refresh_token, authorization_code, bearer_token e.t.c
    let req_body = KeyVal::new().add_list_keyval(vec![
        ("grant_type".into(), "refresh_token".into()),
        ("client_id".into(), client_id.clone()),
        ("refresh_token".into(), redis::cmd("GET").arg(&["refresh_token"]).query_async(&mut con).await.unwrap())
    ]).to_urlencode();

    println!("LEVEL THREE {:#?}", req_body);

    let request = RequestBuilder::new(Method::POST, format!("{}/oauth2/token", twitter_v2))
        .with_basic_auth(client_id, client_secret)
        .with_body(req_body, content).build_request();

        // expected contents - token_type, access_token, scope, expires_in, refresh
    let res = make_request(request, hyper_client.clone()).await;

    if let Ok((_header, body)) = res {
        // should this two changes be moved into an interceptor for string/unpredictable bodies
        // - todo!() exactly the same code is used on the access_token function in controllers/handle_redirect.rs
        let body: HashMap<String, Value> = serde_json::from_slice(&body).unwrap();
        let has_access_token = body.get("access_token");
        let has_refresh_token = body.get("refresh_token");

        if has_access_token.is_some() && has_refresh_token.is_some() {
            // let body_string = String::from_utf8_lossy(&body).to_string();
            println!("THE BODY STRING WITH AN OBJ!!!!!!! {:#?}", body);
            println!("!!!!!!!!!!!!!!!!!!!!!!VERIFIED!!!!!!!!!!!!!!!!!!!!!!");
            let a_t = body.get("access_token").unwrap().clone();
            let r_t = body.get("refresh_token").unwrap().clone();
            let access_token: String = serde_json::from_value(a_t).unwrap();
            let refresh_token: String = serde_json::from_value(r_t).unwrap();
            redis::cmd("SET").arg(&["access_token", &access_token]).query_async(&mut con).await?;
            redis::cmd("SET").arg(&["refresh_token", &refresh_token]).query_async(&mut con).await?;
            return ResponseBuilder::new("Refresh token obtained".into(), Some(""), StatusCode::OK.as_u16()).reply();
        }
    }
    
    
    
    return ResponseBuilder::new("Error connecting to your Twitter account".into(), Some(""), 400).reply();

}