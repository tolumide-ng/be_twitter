use http::{StatusCode};
use hyper::{Request, Method};
use redis::{Client as RedisClient};

use crate::{helpers::{request::HyperClient, keyval::KeyVal, response::{make_request, TResult, ApiBody, ResponseBuilder}}, setup::variables::SettingsVars, middlewares::request_builder::RequestBuilder};

pub async fn refresh_token(_req: Request<hyper::Body>, hyper_client: HyperClient, _redis_client: RedisClient) -> TResult<ApiBody> {
    let SettingsVars {client_id, client_secret, ..} = SettingsVars::new();

    let content = "application/x-www-form-urlencoded";

    // todo()! - Make the Grant_type an enum with From method to convert into string - refresh_token, authorization_code, bearer_token e.t.c
    let req_body = KeyVal::new().add_list_keyval(vec![
        ("grant_type".into(), "refresh_token".into()),
        ("client_id".into(), client_id.clone())
    ]).to_urlencode();

    let request = RequestBuilder::new(Method::POST, "https://api.twitter.com/2/oauth2/token".into())
        .with_basic_auth(client_id, client_secret)
        .with_body(req_body, content).build_request();

    let (_header, _body) = make_request(request, hyper_client.clone()).await?;

    ResponseBuilder::new("Refresh token obtained".into(), Some(""), StatusCode::OK.as_u16()).reply()



    
}