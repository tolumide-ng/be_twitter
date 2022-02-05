
use http::{Method, StatusCode, Request};
use hyper::Body;
use redis::{Client as RedisClient};
use serde::{Serialize, Deserialize};
use sqlx::Statement;


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


#[derive(Serialize, Deserialize, Debug)]
struct Data {
    data: UserLookup
}



pub async fn user_lookup(request: Request<Body>, hyper_client: HyperClient, redis_client: RedisClient) -> TResult<ApiBody> {
    println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
    // todo!() move this to params once route management is migrated to routerify
    let username = request.uri().query().unwrap().split("=").collect::<Vec<_>>()[1];
    let SettingsVars {client_id, client_secret, ..} = SettingsVars::new();
    let mut con = redis_client.get_async_connection().await?;
    println!("THE REQUEST {:#?}", request);
    // if let Err(e) = con {
    //     return ResponseBuilder::new("Internal Server Error".into(), Some(""), 500).reply();
    // }

    let access_token = redis::cmd("GET").arg(&["tolumide_test_access"]).query_async(&mut con).await?;

    let req = RequestBuilder::new(Method::GET, format!("https://api.twitter.com/2/users/by/username/{}", username))
        .with_access_token(access_token).build_request();

    println!("THE REQUEST INFO!!!! {:#?}", req);

    let the_result = make_request(req, hyper_client.clone()).await?;

    let body: UserLookup = serde_json::from_slice(&the_result.1).unwrap();

    println!("YES THE BODY {:#?}", body);

    


    ResponseBuilder::new("Access Granted".into(), Some(""), StatusCode::OK.as_u16()).reply()
}