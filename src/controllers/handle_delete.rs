use std::collections::HashMap;
use hyper::{Body, Request};
use redis::{Client as RedisClient};
use futures::{stream, StreamExt};
use tokio;

use crate::{helpers::{request::HyperClient, response::{TResult, ApiBody, ResponseBuilder}}, middlewares::request_builder::RequestBuilder};

type Ids = HashMap<String, Vec<String>>;
// type ObjectStr = HashMap<String, String>;
struct PostIds(Vec<String>);


impl PostIds {
    pub fn parse(s: Ids) -> Self {
        // let mut detail = "";

        match s.get("ids") {
            Some(ids) => {
                if ids.len() < 1 {
                    // detail = "Ids must contains atleast one post id"
                    panic!("Ids must contain atleast one post Id (type String)")
                }

                return Self(ids.clone())
            }
            None => {
                // detail = "Object must contain an array of string (post ids)"
                panic!("Object must contain an array of string (post ids)")
            }
        }
        // let mut obj: ObjectStr = HashMap::new();
        // obj.insert("errors".into(), detail.into());

        // Err(obj)
    }
}


// rename this module to destory which then contains destory RTs and destory Posts
pub async fn handle_delete(request: Request<Body>, hyper_client: HyperClient, redis_client: RedisClient) -> TResult<ApiBody> {
    let mut con = redis_client.get_async_connection().await?;
    let access_token = redis::cmd("GET").arg(&["tolumide_test_access"]).query_async(&mut con).await?;

    // req body for the ids must be a vector of strings(id of tweets)
    let body = request.into_body();
    let body = hyper::body::to_bytes(body).await?.to_owned();
    let body: Ids = serde_json::from_slice(&body)?;
    println!("THE REQUEST BODY {:#?}", body);

    let post_ids = PostIds::parse(body).0;

    

    
    // let req = RequestBuilder::new(Method::POST, format!("https://api.twitter.com/1.1/statuses/destroy/{}.json"))
    return ResponseBuilder::new("Internal Server Error".into(), Some(""), 500).reply();
}