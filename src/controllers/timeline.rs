use http::{Request, StatusCode, Method};
use hyper::Body;
use redis::{Client as RedisClient};

use crate::{helpers::{request::HyperClient, response::{TResult, ApiBody, ResponseBuilder, make_request}}, middlewares::request_builder::RequestBuilder};


pub async fn get_timeline(request: Request<Body>, hyper_client: HyperClient, redis_client: RedisClient) 
 -> TResult<ApiBody>
{
    // let user_id = request.uri().query().unwrap().split("=").collect::<Vec<_>>()[1];
    // let user_id = redis::cmd("SET").arg(&["tolumide_userid", &user_id]).query_async(&mut con).await?;
    let mut con = redis_client.get_async_connection().await?;
    
    let user_id: String = redis::cmd("GET").arg(&["tolumide_userid"]).query_async(&mut con).await?;
    let access_token = redis::cmd("GET").arg(&["tolumide_test_access"]).query_async(&mut con).await?;

    let req = RequestBuilder::new
        (Method::GET, format!("https://api.twitter.com/2/users/{}/tweets", user_id))
        .with_access_token(access_token).build_request();

    let the_result = make_request(req, hyper_client.clone()).await?;

    ResponseBuilder::new("Ok".into(), Some(""), StatusCode::OK.as_u16()).reply()
}