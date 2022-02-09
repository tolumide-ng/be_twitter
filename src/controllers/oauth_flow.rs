use http::{Request, StatusCode, Method, Response};
use hyper::Body;
use redis::{Client as RedisClient};

use crate::{
    helpers::{
        request::HyperClient, response::{
            TResult, ApiBody, ResponseBuilder, make_request, TwitterResponseVecData}, 
        signature::{OAuth, OAuthAddons}, keypair::KeyPair, keyval::KeyVal,
    }, setup::variables::SettingsVars, middlewares::request_builder::RequestBuilder, interceptor::handle_request::TwitterInterceptor
};



pub async fn request_token(request: Request<Body>, 
    hyper_client: HyperClient, 
    redis_client: RedisClient
) -> TResult<ApiBody> {
    let con = redis_client.get_async_connection().await?;
    let SettingsVars{api_key, api_key_secret, oauth1_callback, ..} = SettingsVars::new();
    let consumer = KeyPair::new(api_key, api_key_secret);
    let callback = OAuthAddons::Callback(oauth1_callback.clone());

    let target_url = "https://api.twitter.com/oauth/request_token";

    let signature = OAuth::new(consumer, None, callback, Method::POST).generate_signature(target_url);
    let content_type = "application/x-www-form-urlencoded";

     let request = RequestBuilder::new(Method::POST, target_url.into())
        .with_query("oauth_callback", &urlencoding::encode(&oauth1_callback))
        .with_access_token("OAuth", signature.to_string())
        .with_body(Body::empty(), content_type)
        .build_request();

    let res = make_request(request, hyper_client).await;

    if let Err(e) = res {
        println!("DOCUMENTING THE ISSUE!!!!! {:#?}", e);
        return ResponseBuilder::new("Error".into(), Some("Could not setup the user"), 403).reply()
    }

    let (_header, body) = res.unwrap();
     let body = String::from_utf8_lossy(&body).to_string();

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

    let frags: Vec<&str> = body.split("&").collect();

    println!("THE COLLECTED FRAGMENTS::::::::: {:#?}", frags);

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