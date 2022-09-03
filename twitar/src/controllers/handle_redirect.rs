use http::Method;
use hyper::{StatusCode};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::{helpers::{
    response::{TResult, ApiBody, make_request, ResponseBuilder}, 
    request::{HyperClient}, keyval::KeyVal, commons::GrantType}, 
    configurations::variables::SettingsVars, errors::response::{TError}, middlewares::request_builder::{RequestBuilder, AuthType}, 
    interceptors::handle_request::{Interceptor, V2TokensType}, startup::server::{AppState}, base_repository::db::{V2User, DB, V1User}
};


async fn access_token(hyper_client: HyperClient, pool: &Pool<Postgres>, _user: &V2User, auth_code: String) -> Result<(), TError> {
    let SettingsVars{client_id, callback_url, client_secret, twitter_url, ..} = SettingsVars::new();
    // let V2User {pkce, user_id, ..} = user.v2_user;
    let user = DB::v2_user(&pool, Uuid::parse_str("1b97475c-4ba1-4ccf-8a62-35baf9ff1075")?).await?;
    let V2User {pkce, user_id, ..} = user.unwrap();
    
    let req_body = KeyVal::new().add_list_keyval(vec![
        ("code".into(), auth_code.clone()),
        ("grant_type".to_string(), GrantType::Authorization.to_string()),
        ("client_id".to_string(), client_id.clone()),
        ("redirect_uri".to_string(), callback_url),
        ("code_verifier".to_string(), pkce.unwrap().clone())
    ]).to_urlencode();

    let content_type = "application/x-www-form-urlencoded";

    let request = RequestBuilder::new(Method::POST, format!("{}/2/oauth2/token", twitter_url))
        .with_auth(AuthType::Basic, format!("{}:{}", client_id, client_secret))
        .with_body(req_body, content_type).build_request();

    let res = Interceptor::intercept(make_request(request, hyper_client.clone()).await);

    if let Some(map) = Interceptor::v2_tokens(res) {
        DB::update_secets(pool, map.get(V2TokensType::Access), map.get(V2TokensType::Refresh), user_id).await?;
        return Ok(())
    }

    return Err(TError::InvalidCredentialError("Required keys are not present".into()))
}


// req: Request<hyper::Body>, hyper_client: HyperClient, redis_client: RedisClient
pub async fn handle_redirect(app_state: AppState) -> TResult<ApiBody> {
    // since this endpoint would be called by the frontend, the <USER> data would be available in the request header. Please note, change the callback URL on twitter developers to the frontend_url
    let AppState {redis, hyper, db_pool, req, env_vars, user, ..} = app_state;
    let SettingsVars{state, api_key, twitter_url, ..} = env_vars;

    
    // let mut conn = redis.get_async_connection().await?;
    
    let query_params = KeyVal::query_params_to_keyval(req.uri())?;
    let is_v1_callback = query_params.every(vec!["oauth_token".into(), "oauth_verifier".into()]);
    
    match is_v1_callback {
        Some(k) => {
            // USER WOULD THE PROVIDED BY THE FRONTEND ONCE THIS IS CONNECTED!!!::::
            //  let V1User { oauth_token, user_id, ..} = &user.as_ref().unwrap().v1_user;

            let v1_user = DB::v1_user(&db_pool, Uuid::parse_str("1b97475c-4ba1-4ccf-8a62-35baf9ff1075")?).await?;
            // let user = DB::v1_user(pool, user_id).await?;
             let V1User { oauth_token, user_id, ..} = v1_user.unwrap();


            if k.validate("oauth_token".into(),oauth_token.clone()) {
                let verifier = k.get("oauth_verifier").unwrap();
                DB::add_oauth_verifier(&db_pool, verifier, user_id.to_owned()).await?;

                let target = format!("{}/oauth/access_token", twitter_url);

                let req = RequestBuilder::new(Method::POST, target)
                    .with_query("oauth_consumer_key", &api_key)
                    .with_query("oauth_token", &oauth_token)
                    .with_query("oauth_verifier", verifier)
                    .build_request();

                let res = make_request(req, hyper.clone()).await;

                if let Ok((_header, body)) = res {
                    let body_string = String::from_utf8_lossy(&body).to_string();
                    let params = KeyVal::string_to_keyval(body_string);

                    if let Some(map) = params {

                        let token = map.get("oauth_token").unwrap().to_string();
                        let secret = map.get("oauth_token_secret").unwrap().to_string();
                        
                        DB::update_v1_secets(&db_pool, token, secret, user_id.to_owned()).await?;
                        
                        return ResponseBuilder::new("Access Granted".into(), Some(""), StatusCode::OK.as_u16()).reply();
                    }
                }

            }

        }
        None => {
            // maybe it is a v2 callback
            let is_v2_callback = query_params.every(vec!["code".into(), "state".into()]);

            if let Some(dict) = is_v2_callback {
                if query_params.validate("state".into(), state) {
                    let code = dict.get("code").unwrap().to_string();
                    access_token(hyper.clone(), &db_pool, &user.unwrap().v2_user, code).await?;

                    return ResponseBuilder::new("Access Granted".into(), Some(""), StatusCode::OK.as_u16()).reply();
                }
            }
        }
    }

    if query_params.every(vec!["denied".into()]).is_some() {
        return ResponseBuilder::new("Unauthorized".into(), Some("Permission denied"), StatusCode::UNAUTHORIZED.as_u16()).reply()
    }

    
    ResponseBuilder::new("Bad request".into(), Some(""), StatusCode::BAD_REQUEST.as_u16()).reply()


}
