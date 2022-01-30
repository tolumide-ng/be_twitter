use http::{StatusCode, Method};
use hyper::{Request, Response, Body};
use redis::{Client as RedisClient};

use crate::{helpers::{request::HyperClient, keyval::KeyVal, response::{ApiResponseBody, TResult, ApiBody, make_request}}, setup::variables::SettingsVars, middlewares::request_builder::RequestBuilder};

pub async fn revoke_token(
    _req: Request<hyper::Body>, hyper_client: HyperClient, redis_client: RedisClient
) -> TResult<ApiBody> {
    // todo() All the environment variables access i.e. SettingsVars should be moved into routes/server.rs
    // where the env variable can then be shared as a controller params
    let SettingsVars{client_id, client_secret, ..} = SettingsVars::new();
    let mut con = redis_client.get_async_connection().await.unwrap();

    let req_body = KeyVal::new().add_list_keyval(vec![
        ("token".into(), redis::cmd("GET").arg(&["tolumide_test_access"]).query_async(&mut con).await?),
        ("client_id".into(), client_id.clone()),
        ("token_type_hint".into(), "access_token".into()),
    ]).to_urlencode();

    let content_type = "application/x-www-form-urlencoded";

    let request = RequestBuilder::new(Method::POST, "https://api.twitter.com/2/oauth2/revoke".into())
        .with_basic_auth(client_id, client_secret)
        .with_body(req_body, content_type).build_request();

    let (_header, body) = make_request(request, hyper_client.clone()).await?;

    let body = serde_json::from_slice(&body)?;

    println!("THE RESPONSE FROM REQUESTING FOR A REVOKE ACCESS {:#?}", body);



    let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    let response_body = Response::builder()
        .status(StatusCode::OK).body(ok_body).unwrap();

    Ok(response_body)
}