use std::{collections::HashMap, sync::{Arc, RwLock}};
use hyper::{StatusCode, Method};
use futures::{stream, StreamExt};
use serde::{Serialize, Deserialize};
use tokio;

use crate::{
    helpers::{response::{TResult, ApiBody, ResponseBuilder, make_request, TwitterResponseData}}, 
    middlewares::request_builder::{RequestBuilder, AuthType}, interceptors::handle_request::Interceptor, configurations::variables::SettingsVars, startup::server::AppState
};

use crate::helpers::db::{TweetType};

#[derive(Debug, Serialize, Deserialize)]
enum TimelineBody {
    Data(Vec<String>),
    Meta(HashMap<String, String>)
}

impl TimelineBody {
    pub fn get(&self) {
        // let ab = self.0;
    }
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

    // take tweets/rts/likes in vectors of 10s or less and save them on the db
    // format data to save it on the database
    let all = res_body.clone();
    let read_all = all.read().unwrap();
    let mut tweets: Vec<&Vec<String>> = vec![];
    let mut rts: Vec<&Vec<String>> = vec![];
    let mut likes: Vec<&Vec<String>> = vec![];
    let tweet_types = vec![TweetType::Rts, TweetType::Tweets, TweetType::Likes];
    const MULTIPLES_OF_TEN: usize = 10;

    for tweet_type in tweet_types {
        let mut map = &*read_all.get(&tweet_type.to_string()).unwrap();
        println!("..........>>>>>>>>>>>>>>........>>>>>>>>>......>>>>>>>>>>>>>>> {:#?}", map);
        
        
        if let TimelineBody::Data(data) = map {
            // we intend to save the ids in an array of 10 ids
            let mut times: usize = data.len()/MULTIPLES_OF_TEN;
            let mut is_multiple = false;
            let mut current = 0;
            
            // let mut ids: &Vec<String> = &Vec::with_capacity(10);

            if times * MULTIPLES_OF_TEN != data.len() {
                // the extra time to get the remainder < 10 into the db
                times += 1;
                is_multiple = true;
            }

            loop {
                times -= 1;
                let ids = &data[current..11].to_vec();
                match tweet_type {
                    TweetType::Rts => { 
                        // 
                     }
                    TweetType::Tweets => {}
                    TweetType::Likes => {}
                }
                break;
            }



            // for ids in 0..times + is_multiples_of_ten {
            //     if is_multiples_of_ten == 1 && ids == data.len() - 1 {

            //     } else {
            //         // let new: Vec<_> = data.splice(0.., vec![]).collect();
            //     }
            // }
        }
    }

    ResponseBuilder::new("Ok".into(), Some(res_body), StatusCode::OK.as_u16()).reply()
}


