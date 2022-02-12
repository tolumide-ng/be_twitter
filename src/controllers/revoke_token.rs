use http::{StatusCode, Method};
use hyper::{Request};
use redis::{Client as RedisClient};
use serde::{Serialize, Deserialize};

use crate::{helpers::{
    request::HyperClient, keyval::KeyVal, 
    response::{TResult, ApiBody, make_request, ResponseBuilder}}, 
    setup::variables::SettingsVars, middlewares::request_builder::RequestBuilder, interceptor::handle_request::Interceptor, app::server::AppState
};



#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    revoked: bool,
}

pub async fn revoke_token(app_state: AppState) -> TResult<ApiBody> {
    let AppState {redis, hyper, ..} = app_state;
    let SettingsVars{client_id, client_secret, twitter_v2, ..} = SettingsVars::new();
    let mut con = redis.get_async_connection().await.unwrap();

    let req_body = KeyVal::new().add_list_keyval(vec![
        ("token".into(), redis::cmd("GET").arg(&["access_token"]).query_async(&mut con).await?),
        ("client_id".into(), client_id.clone()),
        ("token_type_hint".into(), "access_token".into()),
    ]).to_urlencode();

    let content_type = "application/x-www-form-urlencoded";

    let request = RequestBuilder::new(Method::POST, format!("{}/oauth2/revoke", twitter_v2))
        .with_basic_auth(client_id, client_secret)
        .with_body(req_body, content_type).build_request();

    let res = Interceptor::intercept(make_request(request, hyper).await);

    if let Err(e) = res {
        return ResponseBuilder::new("Error".into(), Some(e.0), e.1).reply()
    }
    // let body: ApiResponse = serde_json::from_slice(&body)?;

    ResponseBuilder::new("Access revoked".into(), Some(""), StatusCode::OK.as_u16()).reply()

}