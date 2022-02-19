use hyper::{StatusCode, Method};
use serde::{Serialize, Deserialize};

use crate::{helpers::{
    keyval::KeyVal, response::{TResult, ApiBody, make_request, ResponseBuilder}}, 
    configurations::variables::SettingsVars, middlewares::request_builder::{RequestBuilder, AuthType}, 
    interceptors::handle_request::Interceptor, startup::server::AppState
};



#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    revoked: bool,
}

pub async fn revoke_token(app_state: AppState) -> TResult<ApiBody> {
    let AppState {redis, hyper, env_vars, ..} = app_state;
    let SettingsVars{client_id, client_secret, twitter_url, ..} = env_vars;
    let mut con = redis.get_async_connection().await.unwrap();

    let req_body = KeyVal::new().add_list_keyval(vec![
        ("token".into(), redis::cmd("GET").arg(&["access_token"]).query_async(&mut con).await?),
        ("client_id".into(), client_id.clone()),
        ("token_type_hint".into(), "access_token".into()),
    ]).to_urlencode();

    let content_type = "application/x-www-form-urlencoded";

    let request = RequestBuilder::new(Method::POST, format!("{}/2/oauth2/revoke", twitter_url))
        .with_auth(AuthType::Basic, format!("{}:{}", client_id, client_secret))
        .with_body(req_body, content_type).build_request();

    let res = Interceptor::intercept(make_request(request, hyper).await);

    if let Err(e) = res {
        return ResponseBuilder::new("Error".into(), Some(e.0), e.1).reply()
    }
    // let body: ApiResponse = serde_json::from_slice(&body)?;

    ResponseBuilder::new("Access revoked".into(), Some(""), StatusCode::OK.as_u16()).reply()

}