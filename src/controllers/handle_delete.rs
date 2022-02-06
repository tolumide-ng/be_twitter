use std::{collections::HashMap, io::Read};
use http::Method;
use hyper::{Body, Request};
use redis::{Client as RedisClient};
use futures::{stream, StreamExt};
use tokio;

use crate::{helpers::{request::HyperClient, response::{TResult, ApiBody, ResponseBuilder, make_request}}, middlewares::request_builder::RequestBuilder};

type Ids = HashMap<String, Vec<String>>;
// type ObjectStr = HashMap<String, String>;
struct PostIds(Vec<String>);


impl PostIds {
    pub fn parse(s: Ids) -> Self {
        // let mut detail = "";

        match s.get("ids") {
            Some(ids) => {
                if ids.len() < 1 || ids.len() > 10 {
                    // detail = "Ids must contains atleast one post id"
                    panic!("Ids must contain atleast one and a maximum of 10 post Ids (type String)")
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
    let access_token: String = redis::cmd("GET").arg(&["tolumide_test_access"]).query_async(&mut con).await?;

    // req body for the ids must be a vector of strings(id of tweets)
    let body = request.into_body();
    let body = hyper::body::to_bytes(body).await?.to_owned();
    let body: Ids = serde_json::from_slice(&body)?;
    println!("THE REQUEST BODY {:#?}", body);

    let post_ids = PostIds::parse(body).0;
    let parallel_requests = post_ids.len();

    let bodies = stream::iter(post_ids)
        .map(|id| {
            let client = hyper_client.clone();
            let token = access_token.clone();

            tokio::spawn(async move {
                let request = RequestBuilder::new(Method::POST, format!("https://api.twitter.com/1.1/statuses/unretweet/{}.json", id))
                    .with_access_token(token).build_request();

                let response = make_request(request, client).await.unwrap();
                println!("THE HEADER {:#?}", response.0);
                Ok(response.1)
                // response.1.bytes().await
                // HANDLE RESPONSE HERE ---- IT IS TIME TO CREATE THE INTERCEPTOR THAT HANDLES THE RESPONSE OF THE MAKE_REQUEST CALL
                // let res = make_request(request, client);
            })
        }).buffer_unordered(parallel_requests);

    bodies
        .for_each(|res| async {
            match res {
                Ok(Ok(b)) => {}
                Ok(Err(e)) => {}
                Err(e) => {}
            }
        }).await;




    
    // let req = RequestBuilder::new(Method::POST, format!("https://api.twitter.com/1.1/statuses/destroy/{}.json"))
    return ResponseBuilder::new("Internal Server Error".into(), Some(""), 500).reply();
}