use hyper::{Method, StatusCode};

use crate::{helpers::{
    response::{ResponseBuilder, TResult, ApiBody, make_request, TwitterResponseHashData}}, 
    middlewares::request_builder::{RequestBuilder, AuthType}, 
    interceptors::handle_request::Interceptor, configurations::variables::SettingsVars, startup::server::AppState
};


// use this endpoint to verify the validity of the username when they want to request for their timeline when using OAuth2.0
pub async fn user_lookup(app_state: AppState) -> TResult<ApiBody> {
    // todo!() move this to params once route management is migrated to routerify
    let AppState{redis, req, hyper, env_vars, ..} = app_state;
    let SettingsVars {twitter_url, ..} = env_vars;

    let username = req.uri().query().unwrap().split("=").collect::<Vec<_>>()[1];
    let mut con = redis.get_async_connection().await?;

    let access_token = redis::cmd("GET").arg(&["access_token"]).query_async(&mut con).await?;

    let req = RequestBuilder::new(Method::GET, format!("{}/2/users/by/username/{}", twitter_url, username))
        .with_auth(AuthType::Bearer, access_token).build_request();

    let res= Interceptor::intercept(make_request(req, hyper.clone()).await);

    if let Err(e) = res {
        return ResponseBuilder::new("Error".into(), Some(e.0), e.1).reply();
    }

    let body: TwitterResponseHashData = serde_json::from_value(res.unwrap()).unwrap();
    let user = body.into_one_dict();
    let user_id = user.get("id").unwrap();

    redis::cmd("SET").arg(&["userid", &user_id]).query_async(&mut con).await?;
    ResponseBuilder::new("Ok".into(), Some(""), StatusCode::OK.as_u16()).reply()
}