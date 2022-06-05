use hyper::{Method, StatusCode};

use crate::{helpers::{keyval::KeyVal, response::{make_request, TResult, ApiBody, ResponseBuilder}, commons::GrantType}, 
configurations::variables::SettingsVars, middlewares::request_builder::{RequestBuilder, AuthType}, interceptors::handle_request::{Interceptor, V2TokensType}, startup::server::AppState, base_repository::db::{V2User, DB}};

pub async fn refresh_token(app_state: AppState) -> TResult<ApiBody> {
    let AppState {db_pool, hyper, env_vars, user, ..} = app_state;
    let SettingsVars {client_id, client_secret, twitter_url, ..} = env_vars;

    let V2User {refresh_token, user_id, ..} = user.unwrap().v2_user;
    let content = "application/x-www-form-urlencoded";


    let req_body = KeyVal::new().add_list_keyval(vec![
        ("grant_type".into(), GrantType::Refresh.to_string()),
        ("client_id".into(), client_id.clone()),
        ("refresh_token".into(), refresh_token.unwrap())
    ]).to_urlencode();


    let request = RequestBuilder::new(Method::POST, format!("{}/2/oauth2/token", twitter_url))
        .with_auth(AuthType::Basic, format!("{}:{}", client_id, client_secret))
        .with_body(req_body, content).build_request();

        // expected contents - token_type, access_token, scope, expires_in, refresh
    let res = Interceptor::intercept(make_request(request, hyper.clone()).await);

    if let Some(map) = Interceptor::v2_tokens(res) {
        DB::update_secets(&db_pool, map.get(V2TokensType::Access), map.get(V2TokensType::Refresh), user_id).await?;
        return ResponseBuilder::new("Refresh token obtained".into(), Some(""), StatusCode::OK.as_u16()).reply();
    }

    return ResponseBuilder::new("Error connecting to your Twitter account".into(), Some(""), 400).reply();

}