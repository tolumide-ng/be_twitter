use http::{Request, StatusCode, Method};
use hyper::Body;
use redis::{Client as RedisClient};

use crate::{helpers::{request::HyperClient, 
    response::{TResult, ApiBody, ResponseBuilder, make_request, TwitterResponseVecData}}, 
    middlewares::request_builder::RequestBuilder, interceptor::handle_request::TwitterInterceptor, setup::variables::SettingsVars
};




pub async fn get_timeline(request: Request<Body>, hyper_client: HyperClient, redis_client: RedisClient) 
 -> TResult<ApiBody>
{
    let mut con = redis_client.get_async_connection().await?;

    let SettingsVars { twitter_v2, ..} = SettingsVars::new();
    
    let user_id: String = redis::cmd("GET").arg(&["tolumide_userid"]).query_async(&mut con).await?;
    let access_token = redis::cmd("GET").arg(&["access_token"]).query_async(&mut con).await?;

    let req = RequestBuilder::new
        (Method::GET, format!("{}/users/{}/tweets", twitter_v2, user_id))
        .with_query("max_results", "100")
        .with_access_token("Bearer", access_token).build_request();

    let res = TwitterInterceptor::intercept(make_request(req, hyper_client.clone()).await);

    if let Err(e) = res {
        return ResponseBuilder::new("Error".into(), Some(e.0), e.1).reply()
    }

    let body: TwitterResponseVecData = serde_json::from_value(res.unwrap()).unwrap();

    let parsed = body.separate_tweets_from_rts(true);

    println!("PARSED \n\n {:#?} \n\n ", parsed);

    ResponseBuilder::new("Ok".into(), Some(""), StatusCode::OK.as_u16()).reply()
}