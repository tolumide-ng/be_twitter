use std::{collections::HashMap, borrow::Cow};

use http::{Request, StatusCode, Method, Response};
use hyper::Body;
use redis::{Client as RedisClient};

use crate::{
    helpers::{
        request::HyperClient, response::{
            TResult, ApiBody, ResponseBuilder, make_request}, 
        signature::{OAuth, OAuthAddons}, keypair::KeyPair, keyval::KeyVal,
    }, setup::variables::SettingsVars, middlewares::request_builder::RequestBuilder, interceptor::handle_request::TwitterInterceptor
};



pub async fn request_token(request: Request<Body>, 
    hyper_client: HyperClient, 
    redis_client: RedisClient
) -> TResult<ApiBody> {
    let mut con = redis_client.get_async_connection().await?;
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
        return ResponseBuilder::new("Error".into(), Some("Could not setup the user"), 403).reply()
    }

    let (_header, body) = res.unwrap();
    let body = String::from_utf8_lossy(&body).to_string();
    let oauth: Vec<&str> = body.split("&").collect();

    // We can work with the result of this mapping but in order to make our code readable and easy to debug in future it would
    // be formatted into a vector of KeyVal hashmaps
    let oauth_credentials = oauth.iter().map(|auth| {
        let a: String = auth.to_string();
        let bb = a.split("=").collect::<Vec<&str>>();
        let ccc = bb.iter().map(|a| a.to_string()).collect::<Vec<_>>();
        return ccc
    }).collect::<Vec<_>>();

    let mut map = HashMap::new();
    // this is all to make readability easier when we get to persisting these information
    oauth_credentials.iter().for_each(|a| { map.insert(a[0].clone(), a[1].clone()); });

    println!("THE DC {:#?}", oauth_credentials);
    if let Some(val) = map.get("oauth_callback_confirmed") {
        if val == "true" {
            redis::cmd("SET").arg(&["oauth_token", map.get("oauth_token").unwrap()]).query_async(&mut con).await?;
            redis::cmd("SET").arg(&["oauth_token_secret", map.get("oauth_token_secret").unwrap()]).query_async(&mut con).await?;

            let query_dict = KeyVal::new().add_list_keyval(vec![
                ("oauth_token".to_string(), map.get("oauth_token").unwrap().into())
            ]);
            
            let request = RequestBuilder::new(Method::GET, "https://api.twitter.com/oauth/authorize".into())
                .add_query_params(query_dict)
                .build_request();


            println!("\n\n\n THE TARGET REDIRECT REQUEST {:#?} \n\n\n", request);

            let redirect_to = Response::builder().status(302)
                .header("Location", request.uri().to_string())
                .body(Body::from(request.uri().to_string())).unwrap();

            return Ok(redirect_to)
        }
    }

    return ResponseBuilder::new("Intrernal Server Error".into(), Some("OAuth callback is not confirmed"), 500).reply();

}

