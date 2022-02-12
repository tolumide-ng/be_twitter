use hyper::{Method, StatusCode};

use crate::{helpers::{keyval::KeyVal, response::{make_request, TResult, ApiBody, ResponseBuilder}, commons::GrantType}, setup::variables::SettingsVars, middlewares::request_builder::RequestBuilder, interceptor::handle_request::{Interceptor, V2TokensType}, app::server::AppState};

pub async fn refresh_token(app_state: AppState) -> TResult<ApiBody> {
    let AppState {redis, hyper, env_vars, ..} = app_state;
    let SettingsVars {client_id, client_secret, twitter_v2, ..} = env_vars;

    let mut con = redis.get_async_connection().await.unwrap();
    let content = "application/x-www-form-urlencoded";

    println!("LEVEL TWO");

    // todo()! - Make the Grant_type an enum with From method to convert into string - refresh_token, authorization_code, bearer_token e.t.c
    let req_body = KeyVal::new().add_list_keyval(vec![
        ("grant_type".into(), GrantType::Refresh.to_string()),
        ("client_id".into(), client_id.clone()),
        ("refresh_token".into(), redis::cmd("GET").arg(&["refresh_token"]).query_async(&mut con).await.unwrap())
    ]).to_urlencode();

    println!("LEVEL THREE {:#?}", req_body);

    let request = RequestBuilder::new(Method::POST, format!("{}/oauth2/token", twitter_v2))
        .with_basic_auth(client_id, client_secret)
        .with_body(req_body, content).build_request();

        // expected contents - token_type, access_token, scope, expires_in, refresh
    let res = Interceptor::intercept(make_request(request, hyper.clone()).await);

    if let Some(map) = Interceptor::v2_tokens(res) {
        redis::cmd("SET").arg(&["access_token", &map.get(V2TokensType::Access)]).query_async(&mut con).await?;
        redis::cmd("SET").arg(&["refresh_token", &map.get(V2TokensType::Refresh)]).query_async(&mut con).await?;
        return ResponseBuilder::new("Refresh token obtained".into(), Some(""), StatusCode::OK.as_u16()).reply();
    }

    return ResponseBuilder::new("Error connecting to your Twitter account".into(), Some(""), 400).reply();

}