use http::{Request, StatusCode, Method};
use hyper::Body;
use redis::{Client as RedisClient};

use crate::{helpers::{request::HyperClient, response::{TResult, ApiBody, ResponseBuilder}, signature::{OAuth, OAuthAddons}, keypair::KeyPair, keyval::KeyVal}, setup::variables::SettingsVars, middlewares::request_builder::RequestBuilder};



pub async fn request_token(request: Request<Body>, 
    hyper_client: HyperClient, 
    redis_client: RedisClient
) -> TResult<ApiBody> {
    let con = redis_client.get_async_connection().await?;
    // todo() pass env variables directly to all controllers as function params
    let SettingsVars{client_id, client_secret, redirect_uri, ..} = SettingsVars::new();
    let consumer = KeyPair::new(client_id, client_secret);
    let callback = OAuthAddons::Callback(redirect_uri.clone());

    let target_url = "https://api.twitter.com/oauth/request_token";

    let signature = OAuth::new(consumer, None, callback, Method::POST).generate_signature(target_url);

    println!("THE SIGNATURE {:#?}", signature.to_string());

     let request = RequestBuilder::new(Method::POST, "https://api.twitter.com/oauth/request_token".into())
        .with_query("oauth_callback", &redirect_uri).with_access_token("OAuth", signature.to_string())
        .build_request();

    println!("THE REQUEST PAGE {:#?}", request);

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