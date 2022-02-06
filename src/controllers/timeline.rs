use http::{Request, StatusCode, Method};
use hyper::Body;
use redis::{Client as RedisClient};
use serde_json::Value;

use crate::{helpers::{request::HyperClient, response::{TResult, ApiBody, ResponseBuilder, make_request, TwitterResponseData}}, middlewares::request_builder::RequestBuilder, errors::twitter_errors::TwitterResponseError};




pub async fn get_timeline(request: Request<Body>, hyper_client: HyperClient, redis_client: RedisClient) 
 -> TResult<ApiBody>
{
    let mut con = redis_client.get_async_connection().await?;
    
    let user_id: String = redis::cmd("GET").arg(&["tolumide_userid"]).query_async(&mut con).await?;
    let access_token = redis::cmd("GET").arg(&["tolumide_test_access"]).query_async(&mut con).await?;

    let req = RequestBuilder::new
        (Method::GET, format!("https://api.twitter.com/2/users/{}/tweets", user_id))
        .with_query("max_results", "100")
        .with_access_token(access_token).build_request();

    let res = make_request(req, hyper_client.clone()).await;

    match res {
        Ok(resp) => {
            let (_header, body) = resp;
            let response: Value = serde_json::from_slice(&body)?;

            if response["errors"] != Value::Null {
                let err: TwitterResponseError = serde_json::from_slice(&body)?;
                let detail = err.errors[0].get("detail").unwrap();
                return ResponseBuilder::new(detail.clone(), Some(""), 400).reply();
            }

            let data: TwitterResponseData = serde_json::from_slice(&body)?;

            println!("WHAT THE DATA LOOKS LIKE AFTER PARSING {:#?}", data);
            let parsed = data.separate_tweets_from_rts(true);

            println!("PARSED \n\n {:#?} \n\n ", parsed);

        }
        Err(e) => {
            return ResponseBuilder::new("Internal Server Error".into(), Some(""), 500).reply();
        }
    }

    ResponseBuilder::new("Ok".into(), Some(""), StatusCode::OK.as_u16()).reply()
}