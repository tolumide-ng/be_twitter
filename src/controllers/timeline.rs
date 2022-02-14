use std::{collections::HashMap, sync::{Arc, RwLock}};
use hyper::{StatusCode, Method};
use futures::{stream, StreamExt};
use serde::{Serialize, Deserialize};
use tokio;

use crate::{
    helpers::{response::{TResult, ApiBody, ResponseBuilder, make_request, TwitterResponseData}}, 
    middlewares::request_builder::{RequestBuilder, AuthType}, interceptor::handle_request::Interceptor, setup::variables::SettingsVars, app::server::AppState
};

use super::destroy::TweetType;

#[derive(Debug, Serialize, Deserialize)]
enum TimelineBody {
    Data(Vec<String>),
    Meta(HashMap<String, String>)
}


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

    let bodies = stream::iter(requests).map(|tweet| {
        let client = hyper.clone();

        let req = match tweet {
            TweetType::Tweets | TweetType::Rts => {
                get_url("tweets").with_query("max_results", MAX_TWEETS).build_request()
            }
            TweetType::Likes => {
                get_url("liked_tweets").with_query("max_results", MAX_TWEETS).build_request()
            }
        };


        tokio::spawn(async move {
            let response = Interceptor::intercept(make_request(req, client).await);
            (tweet, response)
        })
    }).buffer_unordered(2);



    let res_body: Arc<RwLock<HashMap<String, TimelineBody>>> = Arc::new(RwLock::new(HashMap::new()));

    bodies.for_each(|res| async {
        // let mut response: HashMap<String, TimelineBody> = HashMap::new();
        match res {
            Ok(body) => {
                let dic_body: TwitterResponseData = serde_json::from_value(body.1.unwrap()).unwrap();
                
                println!("\n\n THE VEC BODY {:#?} \n\n", dic_body);
                
                if body.0 == TweetType::Tweets {
                    let parsed_body = dic_body.separate_tweets_from_rts(true);

                    let keys = parsed_body.keys().cloned().collect::<Vec<_>>();
                    let tweets_and_rts = keys.iter()
                        .map(|k| (k.into(), TimelineBody::Data(parsed_body.get(k).unwrap().to_vec())))
                        .collect::<HashMap<String, TimelineBody>>();

                    let mut res_body = res_body.write().unwrap();
                    res_body.extend(tweets_and_rts);
                    // res_body.get().extend(tweets_and_rts);
                    // this point should mean inserting it into the db
                } else {
                    // If the tweet response is a dictionary, get all the ids of the likes
                    let like_ids = dic_body.get_ids();
                    let mut res_body = res_body.write().unwrap();
                    res_body.insert("likes".into(), TimelineBody::Data(like_ids));
                }

                let meta = dic_body.parse_metadata();
                let meta_name = format!("meta_{}", body.0.to_string());
                let mut res_body = res_body.write().unwrap();
                res_body.insert(meta_name, TimelineBody::Meta(meta));
            }
            Err(e) => {
                // errors = true;
                // how do I intend to handle the errors in this case????
                eprintln!("ERROR LELEYI {:#?}", e);
            }
        }
    }).await;

    ResponseBuilder::new("Ok".into(), Some(res_body), StatusCode::OK.as_u16()).reply()
}


