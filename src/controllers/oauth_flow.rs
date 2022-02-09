use http::{Request, StatusCode, Method, Response};
use hyper::Body;
use redis::{Client as RedisClient};
use serde_json::Value;

use crate::{
    helpers::{
        request::HyperClient, response::{
            TResult, ApiBody, ResponseBuilder, make_request, TwitterResponseVecData}, 
        signature::{OAuth, OAuthAddons}, keypair::KeyPair, keyval::KeyVal, utils::percent_encode
    }, setup::variables::SettingsVars, middlewares::request_builder::RequestBuilder, interceptor::handle_request::TwitterInterceptor
};



pub async fn request_token(request: Request<Body>, 
    hyper_client: HyperClient, 
    redis_client: RedisClient
) -> TResult<ApiBody> {
    let con = redis_client.get_async_connection().await?;
    // todo() pass env variables directly to all controllers as function params
    let SettingsVars{api_key, api_key_secret, redirect_uri, ..} = SettingsVars::new();
    let consumer = KeyPair::new(api_key, api_key_secret);
    // let cb_url = urlencoding::encode(redirect_uri.clone());
    let callback = OAuthAddons::Callback(redirect_uri.clone());

    let target_url = "https://api.twitter.com/oauth/request_token";
    // let target_url = "https://api.twitter.com/1.1/statuses/update.json";

    let signature = OAuth::new(consumer, None, callback, Method::POST).generate_signature(target_url);

    println!("THE SIGNATURE {:#?}", signature.to_string());
    let content_type = "application/x-www-form-urlencoded";

     let request = RequestBuilder::new(Method::POST, target_url.into())
        .with_query("oauth_callback", &percent_encode(&redirect_uri).to_string())
        .with_access_token("OAuth", signature.to_string())
        // .with_body(Body::empty(), content_type)
        .build_request();

    println!("\n\n THE REQUEST PAGE {:#?} \n\n", request);

    let response_body = Response::builder().status(307)
        .header("Location", request.uri().to_string())
        .body(Body::empty()).unwrap();

    let res = TwitterInterceptor::intercept(make_request(request, hyper_client).await);

    if let Err(e) = res {
        eprintln!("::>>>>>>><<<<<<<<<<<<:: {:#?}", e);

        return ResponseBuilder::new("Error".into(), Some(""), 400).reply()
    }

    let body: TwitterResponseVecData = serde_json::from_value(res.unwrap()).unwrap();

    // let body = String::from_utf8_lossy(&body);
    println!("
    \n\n
    *******************************************************
    *******************************************************
    {:#?}
    *******************************************************
    *******************************************************
    \n\n
    ", body);

    // Ok(response_body)

    

    return ResponseBuilder::new("Ok".into(), Some(""), 200).reply();
}


pub async fn handle_redirect(req: Request<hyper::Body>, hyper_client: HyperClient, redis_client: RedisClient) -> TResult<ApiBody> {
    let SettingsVars{state, ..} = SettingsVars::new();

    println!("EVERYTHING ABOUT THE REQUEST TO THIS ENDPOINT {:#?}", req);

    let query_params = KeyVal::query_params_to_keyval(req.uri())?
        .to_access_token()?.validate_state(state)?;

   

    // Make request to POST the access token
    // access_token(hyper_client.clone(), redis_client, query_params.code).await?;

    ResponseBuilder::new("Access Granted".into(), Some(""), StatusCode::OK.as_u16()).reply()
}