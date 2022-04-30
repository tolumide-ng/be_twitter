use std::{collections::HashMap};
use hyper::{Body, Request, Method};
use futures::{stream, StreamExt};
use serde_json::Value;
use tokio;

use crate::{
    helpers::{
        response::{
            TResult, ApiBody, ResponseBuilder, make_request
        }, signature::{
            OAuth, OAuthAddons
        }, keypair::KeyPair
    }, middlewares::request_builder::{RequestBuilder, AuthType}, configurations::variables::SettingsVars, startup::server::AppState, base_repository::db::V2User
};
use crate::helpers::db::{TweetType};

type Ids = HashMap<String, Vec<String>>;


const MAX_SINGLE_DELETES: usize = 50;

#[derive(Debug, Clone)]
struct PostIds(Vec<(String, TweetType)>);


// FIND A BETTER WAY TO HANDLE THIS PARSING, SO THE CODE IS MORE READABLE AND EASIER TO FOLLOW
impl PostIds {
    pub fn parse(s: Ids) -> Self {
        let received_keys = s.keys().cloned().collect::<Vec<String>>();
        let expected_keys = [TweetType::Rts, TweetType::Tweets, TweetType::Likes];

        if received_keys.contains(&TweetType::Rts.to_string()) && received_keys.contains(&&TweetType::Tweets.to_string()) {
            let total_rts = s.get(&TweetType::Rts.to_string()).unwrap().len();
            let total_tweets = s.get(&TweetType::Tweets.to_string()).unwrap().len();
            let total_likes = s.get(&TweetType::Likes.to_string()).unwrap().len();


            if *[total_rts, total_tweets, total_likes].iter().max().unwrap() > MAX_SINGLE_DELETES {
                panic!("Total tweets or rts cannot be more than 50")
            }

            let mut all_ids: Vec<(String, TweetType)> = vec![];

            for key in expected_keys {
                let ids = s.get(&key.to_string()).unwrap();

                let duplicates = ids.iter()
                    .find(|x| ids.iter().filter(|y| x == y).count() >= 2);

                let empty_string = ids.iter().find(|x| x.len() < 1);

                let not_number = ids.iter().find(|id| id.parse::<u64>().is_err());

                if duplicates.is_some() || empty_string.is_some() || not_number.is_some() {
                    panic!("{} must be an array of ids or an empty array", key)
                }

                let ids = s.get(&key.to_string()).unwrap()
                    .iter().map(|k| (k.clone(), key)).collect::<Vec<(String, TweetType)>>();

                all_ids.extend(ids);

            }

            return Self(all_ids)
        }

        panic!("request object must contain rts and tweets with an array of string as values")
    }
}


// rename this module to destory which then contains destory RTs and destory Posts
pub async fn handle_delete(app_state: AppState) -> TResult<ApiBody> {
    let AppState {env_vars, req, hyper, user, ..} = app_state;
    let SettingsVars { twitter_url, .. } = env_vars;

    let V2User {access_token, user_id, ..} = user.unwrap().v2_user;

    // let mut con = redis.get_async_connection().await?;
    // let access_token: String = redis::cmd("GET").arg(&["access_token"]).query_async(&mut con).await?;


    // req body for the ids must be a vector of strings(id of tweets)
    let req_body = req.into_body();
    let  byte_body = hyper::body::to_bytes(req_body).await?.to_owned();
    let body: Ids = serde_json::from_slice(&byte_body).unwrap();
    // let user_id: String = redis::cmd("GET").arg(&["userid"]).query_async(&mut con).await.unwrap();

    let post_ids = PostIds::parse(body).0;
    let parallel_requests = post_ids.len();
    
    let bodies = stream::iter(post_ids)
    .map(|id: (String, TweetType)| {
        let client = hyper.clone();
        let token = access_token.as_ref().unwrap().clone();
        
        let twitter_url = twitter_url.clone();
        let mut request: Option<Request<Body>> = None;
        
        // At the moment, twitter uses OAuth1.0 and 2.0 for Delete Tweets while it only uses 1.0 Authentication for its Unretweets which is a v2 endpoint
        // I really love the OAuth2.0 implementation but need backup code for OAuth1.0 (to handle undo retweets)
        match id.1 {
            TweetType::Tweets => {
                request = Some(RequestBuilder::new(Method::DELETE, format!("{}/2/tweets/{}", twitter_url, id.0))
                    .with_auth(AuthType::Bearer, token).build_request());
            }
            TweetType::Rts => {
                request = Some(RequestBuilder::new(Method::DELETE, format!("{}/2/users/{}/retweets/{}", twitter_url, user_id, id.0))
                    .with_auth(AuthType::Bearer, token).build_request());
            }
            TweetType::Likes => {
                request = Some(RequestBuilder::new(Method::DELETE, format!("{}/2/users/{}/likes/{}", twitter_url, user_id, id.0))
                    .with_auth(AuthType::Bearer, token).build_request());
            }
        };


        tokio::spawn(async move {
                let response = make_request(request.unwrap(), client).await.unwrap();
                response.1
            })
        }).buffer_unordered(parallel_requests);

    bodies
        .for_each(|res| async {
            match res {
                Ok(body) => {
                    let body: Value = serde_json::from_slice(&body).unwrap();
                    // The success body in responsebuilder should include the deleted ids?
                    println!("THE BODY {:#?}", body)
                }
                Err(e) => {
                    // includes failed ids in the responsebuilder body?
                    eprintln!("ERROR {:#?}", e)
                }
            }
        }).await;

    return ResponseBuilder::new("Ok".into(), Some(""), 200).reply();
}