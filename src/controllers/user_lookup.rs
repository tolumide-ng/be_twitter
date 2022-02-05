
use http::{Method, StatusCode};
use redis::{Client as RedisClient};
use serde::{Serialize, Deserialize};


use crate::{helpers::{
    request::HyperClient, 
    response::{
        ResponseBuilder, TResult, ApiBody, CONTENT_TYPE, make_request}
    }, setup::variables::SettingsVars, middlewares::request_builder::RequestBuilder
};


#[derive(Serialize, Deserialize, Debug)]
struct UserLookup {
    id: String,
    name: String,
    username: String,
}



    pub async fn user_lookup(hyper_client: HyperClient, redis_client: RedisClient) -> TResult<ApiBody> {
    let SettingsVars {client_id, client_secret, ..} = SettingsVars::new();
    let mut conn = redis_client.get_async_connection().await;
    
    if let Err(e) = conn {
        return ResponseBuilder::new("Internal Server Error".into(), Some(""), 500).reply();
    }

    let req = RequestBuilder::new(Method::GET, "https://api.twitter.com/2/users/by/username/".into())
        .with_basic_auth(client_id, client_secret).build_request();

    let the_result = make_request(req, hyper_client.clone()).await?;

    let body: UserLookup = serde_json::from_slice(&the_result.1).unwrap();

    println!("YES THE BODY {:#?}", body);

    


    ResponseBuilder::new("Access Granted".into(), Some(""), StatusCode::OK.as_u16()).reply()
}