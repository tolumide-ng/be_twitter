use std::collections::HashMap;

use hyper::{Body, Response, StatusCode, Request};
use redis::{Client as RedisClient};
use crate::{helpers::{
    response::{TResult, ApiResponseBody, ApiBody}, 
    request::{HyperClient}, keyval::KeyVal}, 
    setup::variables::SettingsVars, errors::response::TError
};



#[derive(Debug, Clone)]
pub struct AccessToken {
    pub state: String,
    pub  code: String,
}

impl AccessToken {
    pub fn validate_state(self, local_state: String) -> TResult<Self> {
        if self.state != local_state {
            return Err(TError::InvalidCredential("The state value obtained from the redirect uri does not match the local one"));
        }

        Ok(self)
    }
}
// todo() - I should move all the controllers used to handle 2.0 authentication into one struct and represent them as methods within the struct
pub async fn handle_redirect(req: Request<hyper::Body>, hyper_client: &HyperClient, redis_client: RedisClient) -> TResult<ApiBody> {
    let SettingsVars{state, ..} = SettingsVars::new();

    let query_params = KeyVal::query_params_to_keyval(req.uri())?
        .to_access_token()?.validate_state(state)?;


    let mut con = redis_client.get_async_connection().await.unwrap();

    // connection.get("test");
    let result = redis::cmd("MGET")
        .arg(&["key1"])
        .query_async(&mut con)
        .await?;

    println!("THE RESULT OF THE REDIS QUERY {:#?}", result);
    

     let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    let response_body = Response::builder()
        .status(StatusCode::OK).body(ok_body).unwrap();

    Ok(response_body)
}