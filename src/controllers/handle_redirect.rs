use http::Method;
use hyper::{Body, Response, StatusCode, Request};
use redis::{Client as RedisClient};
use crate::{helpers::{
    response::{TResult, ApiResponseBody, ApiBody, make_request}, 
    request::{HyperClient}, keyval::KeyVal}, 
    setup::variables::SettingsVars, errors::response::{TError}, middlewares::request_builder::RequestBuilder
};


#[derive(Debug, Clone)]
pub struct AccessToken {
    pub state: String,
    pub  code: String,
}

impl AccessToken {
    pub fn validate_state(self, local_state: String) -> TResult<Self> {
        if self.state != local_state {
            return Err(TError::InvalidCredentialError("The state value obtained from the redirect uri does not match the local one"));
        }

        Ok(self)
    }
}

async fn access_token(hyper_client: HyperClient, redis_client: RedisClient, auth_code: String) -> TResult<ApiBody> {
    let SettingsVars{client_id, redirect_uri, client_secret, ..} = SettingsVars::new();
    let mut con = redis_client.get_async_connection().await.unwrap();


    let req_body = KeyVal::new().add_list_keyval(vec![
        ("code".into(), auth_code.clone()),
        ("grant_type".to_string(), "authorization_code".into()),
        ("client_id".to_string(), client_id.clone()),
        ("redirect_uri".to_string(), redirect_uri),
        ("code_verifier".to_string(), redis::cmd("GET").arg(&["tolumide_test_pkce"]).query_async(&mut con).await?)
    ]).to_urlencode();

    let content_type = "application/x-www-form-urlencoded";

    let request = RequestBuilder::new(Method::POST, "https://api.twitter.com/2/oauth2/token".into())
        .with_basic_auth(client_id, client_secret)
        .with_body(req_body, content_type).build_request();

    println!("||||||||||\n\n {:#?} \n\n |||||||||||||||||||", request);

    let (_header, body) = make_request(request, hyper_client.clone()).await?;

    struct AppAccess {
        token_type: String,
        expires_in: i32,
        access_token: String,
        scope: String,
        refresh_token: String,
    }

    let body: AppAccess = serde_json::from_slice(&body)?;
        
    println!("\n\n THE DESERIALIZED BODY \n\n {:#?} \n", body);

    let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    let response_body = Response::builder()
        .status(StatusCode::OK).body(ok_body).unwrap();

    Ok(response_body)
}



pub async fn handle_redirect(req: Request<hyper::Body>, hyper_client: HyperClient, redis_client: RedisClient) -> TResult<ApiBody> {
    let SettingsVars{state, ..} = SettingsVars::new();

    let query_params = KeyVal::query_params_to_keyval(req.uri())?
        .to_access_token()?.validate_state(state)?;

    // Make request to POST the access token
    access_token(hyper_client.clone(), redis_client, query_params.code).await?;

     let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("from me to you".to_string())));

    let response_body = Response::builder()
        .status(StatusCode::OK).body(ok_body).unwrap();

    Ok(response_body)
}