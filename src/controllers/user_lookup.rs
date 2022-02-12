use http::Request;
use hyper::{Method, StatusCode, Body};
use routerify::prelude::*;

use crate::{helpers::{
    response::{ResponseBuilder, TResult, ApiBody, make_request, TwitterResponseHashData}}, 
    middlewares::request_builder::{RequestBuilder, AuthType}, 
    interceptor::handle_request::Interceptor, setup::variables::SettingsVars, app::server::AppState
};


// use this endpoint to verify the validity of the username when they want to request for their timeline when using OAuth2.0
pub async fn user_lookup(req: Request<Body>) -> TResult<ApiBody> {

    let app_state = req.data::<AppState>().unwrap();
    // todo!() move this to params once route management is migrated to routerify
    let AppState{redis, hyper, env_vars, ..} = app_state;
    let SettingsVars {twitter_v2, ..} = env_vars;

    let username = req.uri().query().unwrap().split("=").collect::<Vec<_>>()[1];
    let mut con = redis.get_async_connection().await?;

    let access_token = redis::cmd("GET").arg(&["access_token"]).query_async(&mut con).await?;

    let req = RequestBuilder::new(Method::GET, format!("{}/users/by/username/{}", twitter_v2, username))
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