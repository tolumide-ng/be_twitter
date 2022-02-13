use http::Request;
use hyper::{StatusCode, Method, Body};
use futures::{stream, StreamExt};
use tokio;

use crate::{
    helpers::{response::{TResult, ApiBody, ResponseBuilder, make_request, TwitterResponseVecData}}, 
    middlewares::request_builder::{RequestBuilder, AuthType}, interceptor::handle_request::Interceptor, setup::variables::SettingsVars, app::server::AppState
};

use super::destroy::TweetType;


const MAX_TWEETS: &'static str = "100";

pub async fn get_timeline(app_state: AppState) -> TResult<ApiBody> {
    let AppState {redis, hyper, env_vars, ..} = app_state;
    let SettingsVars { twitter_url, ..} = env_vars;
    
    let mut con = redis.get_async_connection().await?;

    let user_id: String = redis::cmd("GET").arg(&["userid"]).query_async(&mut con).await?;
    let access_token: String = redis::cmd("GET").arg(&["access_token"]).query_async(&mut con).await?;

    let get_url = |path: &'static str| -> RequestBuilder {
        RequestBuilder::new
        (Method::GET, format!("{}/2/users/{}/{}", twitter_url, user_id, path))
        .with_auth(AuthType::Bearer, access_token.clone())
    };

    let requests = vec![TweetType::Tweets, TweetType::Likes];
    let requests_len = requests.len();

    let bodies = stream::iter(requests).map(|tweet| {
        let client = hyper.clone();

        let req = match tweet {
            TweetType::Tweets | TweetType::Rts => {
                get_url("tweets").with_query("max_results", MAX_TWEETS).build_request()
            }
            TweetType::Likes => {
                get_url("liked_tweets").build_request()
            }
        };

        println!("REQUESTS----------------------- {:#?}", req);

        tokio::spawn(async move {
            let response = Interceptor::intercept(make_request(req, client).await);
            (tweet, response)
        })
    }).buffer_unordered(2);

    // let mut response = 

    // let res = Interceptor::intercept(make_request(get_tweets_and_rts, hyper).await);

    // if let Err(e) = res {
    //     return ResponseBuilder::new("Error".into(), Some(e.0), e.1).reply()
    // }
    // let mut errors = false;

    bodies.for_each(|res| async {
        match res {
            Ok(body) => {
                // let b = body.0;

                println!("THE RESPONSE OBTAINED!!!!!!!!!!!!!!!!! {:#?}", body);
                
                let vec_body: TwitterResponseVecData = serde_json::from_value(body.1.unwrap()).unwrap();

                println!("\n\n THE VEC BODY {:#?} \n\n", vec_body);
                
                if vec![TweetType::Rts, TweetType::Tweets].contains(&body.0) {
                    let parsed_body = vec_body.separate_tweets_from_rts(true);
                    println!("THE PARSED BODY {:#?}", parsed_body);
                }
            }
            Err(e) => {
                // errors = true;
                eprintln!("ERROR LELEYI {:#?}", e);
            }
        }
    }).await;

    // let body: TwitterResponseVecData = serde_json::from_value(res.unwrap()).unwrap();

    // let parsed = body.separate_tweets_from_rts(true);

    ResponseBuilder::new("Ok".into(), Some("parsed"), StatusCode::OK.as_u16()).reply()
}