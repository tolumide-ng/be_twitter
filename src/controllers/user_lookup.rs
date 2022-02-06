use http::{Method, StatusCode, Request};
use hyper::Body;
use redis::{Client as RedisClient};
use serde_json::{Value};


use crate::{helpers::{
    request::HyperClient, 
    response::{
        ResponseBuilder, TResult, ApiBody, make_request, TwitterResponseData}
    }, middlewares::request_builder::RequestBuilder, errors::twitter_errors::TwitterResponseError
};


// use this endpoint to verify the validity of the username when they want to request for their timeline
pub async fn user_lookup(request: Request<Body>, hyper_client: HyperClient, redis_client: RedisClient) -> TResult<ApiBody> {
    // todo!() move this to params once route management is migrated to routerify
    let username = request.uri().query().unwrap().split("=").collect::<Vec<_>>()[1];
    let mut con = redis_client.get_async_connection().await?;

    let access_token = redis::cmd("GET").arg(&["tolumide_test_access"]).query_async(&mut con).await?;

    let req = RequestBuilder::new(Method::GET, format!("https://api.twitter.com/2/users/by/username/{}", username))
        .with_access_token(access_token).build_request();

    let res= make_request(req, hyper_client.clone())
        .await;

    // todo!() -Make this into a wrapper method that wraps and decides on all return values of the make_request function
    // this should mean handling only the two possible OK cases;
    match res {
        Ok(resp) => {
            let (_header, body) = resp;
            let response: Value = serde_json::from_slice(&body)?;

            if Value::Null != response["errors"] {
                let err: TwitterResponseError = serde_json::from_slice(&body)?;
                let detail = err.errors[0].get("detail").unwrap();
                return ResponseBuilder::new(detail.clone(), Some(""), 400).reply();
            }


            let data: TwitterResponseData = serde_json::from_slice(&body)?;
            let user = data.into_one_dict();
            let user_id = user.get("id").unwrap();

            redis::cmd("SET").arg(&["tolumide_userid", &user_id]).query_async(&mut con).await?;
            ResponseBuilder::new("Ok".into(), Some(""), StatusCode::OK.as_u16()).reply()

        }
        Err(e) => {
            println!("THE ERRORS {:#?}", e);
            return ResponseBuilder::new("Internal Server Error".into(), Some(""), 500).reply();
        }
    }

}