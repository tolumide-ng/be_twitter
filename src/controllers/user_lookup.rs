use std::{collections::HashMap};
use http::{Method, StatusCode, Request};
use hyper::Body;
use redis::{Client as RedisClient};
use serde::{Serialize, Deserialize};


use crate::{helpers::{
    request::HyperClient, 
    response::{
        ResponseBuilder, TResult, ApiBody, make_request}
    }, middlewares::request_builder::RequestBuilder
};


#[derive(Serialize, Deserialize, Debug)]
struct UserLookup {
    id: String,
    name: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    data: UserLookup
}

impl Data {
    pub fn into_dict(self) -> HashMap<String, String> {
        let mut dict = HashMap::new();
        let UserLookup { id, name, username } = self.data;
        dict.insert("id".into(), id);
        dict.insert("name".into(), name);
        dict.insert("username".into(), username);

        dict
    }
}



pub async fn user_lookup(request: Request<Body>, hyper_client: HyperClient, redis_client: RedisClient) -> TResult<ApiBody> {
    // todo!() move this to params once route management is migrated to routerify
    let username = request.uri().query().unwrap().split("=").collect::<Vec<_>>()[1];
    let mut con = redis_client.get_async_connection().await?;

    // if let Err(e) = con {
    //     return ResponseBuilder::new("Internal Server Error".into(), Some(""), 500).reply();
    // }

    let access_token = redis::cmd("GET").arg(&["tolumide_test_access"]).query_async(&mut con).await?;

    let req = RequestBuilder::new(Method::GET, format!("https://api.twitter.com/2/users/by/username/{}", username))
        .with_access_token(access_token).build_request();

    let the_result = make_request(req, hyper_client.clone()).await?;

    let body: Data = serde_json::from_slice(&the_result.1).unwrap();

    let user = body.into_dict();
    let user_id = user.get("id").unwrap();
    
    redis::cmd("SET").arg(&["tolumide_userid", &user_id]).query_async(&mut con).await?;
    ResponseBuilder::new("Ok".into(), Some(""), StatusCode::OK.as_u16()).reply()
}