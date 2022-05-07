use std::{collections::HashMap};
use hyper::{Method, Response, Body};
use uuid::Uuid;

use crate::{
    helpers::{
        response::{TResult, ApiBody, ResponseBuilder, make_request}, 
        signature::{OAuth, OAuthAddons}, keypair::KeyPair, keyval::KeyVal,
    }, configurations::variables::SettingsVars, middlewares::request_builder::{RequestBuilder, AuthType}, startup::server::AppState, base_repository::db::{DB, V1User},
};



pub async fn request_token(app_state: AppState) -> TResult<ApiBody> {
    let AppState {_, hyper, db_pool, env_vars, ..} = app_state;
    // let mut con = redis.get_async_connection().await?;
    let SettingsVars{api_key, api_key_secret, callback_url, twitter_url, ..} = env_vars;

    // println!("THE CONTENT {:#?}", user);
    
    // TODO == GET the user_id from the request header i.e app_state.req.headers of the frontend
    let user_id = Uuid::parse_str("1b97475c-4ba1-4ccf-8a62-35baf9ff1075")?;

    let consumer = KeyPair::new(api_key, api_key_secret);
    let callback = OAuthAddons::Callback(callback_url.clone());

    let target_url = format!("{}/oauth/request_token", twitter_url);
    let signature = OAuth::new(consumer, None, callback, Method::POST).generate_signature(target_url.clone());
    let content_type = "application/x-www-form-urlencoded";

     let request = RequestBuilder::new(Method::POST, target_url)
        .with_query("oauth_callback", &urlencoding::encode(&callback_url))
        .with_auth(AuthType::OAuth, signature.to_string())
        .with_body(Body::empty(), content_type)
        .build_request();

    let res = make_request(request, hyper).await;

    if let Err(_e) = res {
        return ResponseBuilder::new("Error".into(), Some("Could not setup the user"), 403).reply()
    }

    let (_header, body) = res.unwrap();
    let body = String::from_utf8_lossy(&body).to_string();
    println!("THE RESPONSE BODY::::::::: {:#?}", body);
    let oauth: Vec<&str> = body.split("&").collect();

    // We can work with the result of this mapping but in order to make our code readable and easy to debug in future it would
    // be formatted into a vector of KeyVal hashmaps
    let oauth_credentials = oauth.iter().map(|auth| {
        let a: String = auth.to_string();
        let bb = a.split("=").collect::<Vec<&str>>();
        let ccc = bb.iter().map(|a| a.to_string()).collect::<Vec<_>>();
        return ccc
    }).collect::<Vec<_>>();

    println!("THE OAUTH CREDENTIALS------------------------------------>>>>>>>>>>>>>>>>>>>>>>>>>>|||||||||||||||||||| {:#?}", oauth_credentials);

    let mut map = HashMap::new();
    // this is all to make readability easier when we get to persisting these information
    oauth_credentials.iter().for_each(|a| { map.insert(a[0].clone(), a[1].clone()); });

    println!("AFTER THE ITER WITH FOR EACH");

    if let Some(val) = map.get("oauth_callback_confirmed") {
        println!("FIRST IF!!!!");
        if val == "true" {
            // redis::cmd("SET").arg(&["oauth_token", map.get("oauth_token").unwrap()]).query_async(&mut con).await?;
            // redis::cmd("SET").arg(&["oauth_token_secret", map.get("oauth_token_secret").unwrap()]).query_async(&mut con).await?;
            println!("CONTENT OF THE MAP:::::::::::::::::::::: {:#?}", map);
            let oauth_token = map.get("oauth_token").unwrap().to_string();
            let oauth_secret = map.get("oauth_token_secret").unwrap().to_string();

            DB::create_v1_secets(&db_pool, user_id, oauth_token, oauth_secret).await.unwrap();

            println!("persistsed:::::::::::::::::::::::::>>>>>>>>>>>>>>>>>>>>>>>>>>>");

            let query_dict = KeyVal::new().add_list_keyval(vec![
                ("oauth_token".to_string(), map.get("oauth_token").unwrap().into())
            ]);

            println!("THE QUERY DICTIONARY {:#?}", query_dict);
            
            let request = RequestBuilder::new(Method::GET, format!("{}/oauth/authorize", twitter_url))
                .add_query_params(query_dict)
                .build_request();

            let redirect_to = Response::builder().status(302)
                .header("Location", request.uri().to_string())
                .body(Body::from(request.uri().to_string())).unwrap();


                println!("ABOUTA REDIRECT::::::::::::::::::::::::::::::::::::::::::::::: {:#?}", redirect_to);

            return Ok(redirect_to)
        }
    }

    return ResponseBuilder::new("Internal Server Error".into(), Some("OAuth callback is not confirmed"), 500).reply();

}

