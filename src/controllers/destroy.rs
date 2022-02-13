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
    }, middlewares::request_builder::{RequestBuilder, AuthType}, setup::variables::SettingsVars, app::server::AppState
};

type Ids = HashMap<String, Vec<String>>;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TweetType {
    Tweets,
    Rts,
    Likes,
}

impl std::fmt::Display for TweetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tweets => write!(f, "tweets"),
            Self::Rts => write!(f, "rts"),
            Self::Likes => write!(f, "likes"),
        }
    }
}


const MAX_SINGLE_DELETES: usize = 50;

#[derive(Debug, Clone)]
struct PostIds(Vec<(String, TweetType)>);


// FIND A BETTER WAY TO HANDLE THIS PARSING, SO THE CODE IS MORE READABLE AND EASIER TO FOLLOW
impl PostIds {
    pub fn parse(s: Ids) -> Self {
        let received_keys = s.keys().cloned().collect::<Vec<String>>();
        let expected_keys = [TweetType::Rts, TweetType::Tweets];

        if received_keys.contains(&TweetType::Rts.to_string()) && received_keys.contains(&&TweetType::Tweets.to_string()) {
            let total_rts = s.get(&TweetType::Rts.to_string()).unwrap().len();
            let total_tweets = s.get(&TweetType::Tweets.to_string()).unwrap().len();

            if total_rts> MAX_SINGLE_DELETES || total_tweets > MAX_SINGLE_DELETES {
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
    let AppState {env_vars, redis, req, hyper, ..} = app_state;
    let SettingsVars { api_key, api_key_secret,twitter_url, .. } = env_vars;

    let mut con = redis.get_async_connection().await?;
    let access_token: String = redis::cmd("GET").arg(&["access_token"]).query_async(&mut con).await?;

    // req body for the ids must be a vector of strings(id of tweets)
    let req_body = req.into_body();
    let  byte_body = hyper::body::to_bytes(req_body).await?.to_owned();
    let body: Ids = serde_json::from_slice(&byte_body)?;

    let post_ids = PostIds::parse(body).0;
    let parallel_requests = post_ids.len();

    let oauth_token_key: String = redis::cmd("GET").arg(&["oauth_token"]).query_async(&mut con).await.unwrap();
    let oauth_token_secret = redis::cmd("GET").arg(&["oauth_token_secret"]).query_async(&mut con).await.unwrap();
    // let oauth_consumer_key = redis::cmd("GET").arg(&["oauth_consumer_key"]).query_async(&mut con).await.unwrap();
    
    
    let oauth_token = KeyPair::new(oauth_token_key.clone(), oauth_token_secret);
    let consumer = KeyPair::new(api_key, api_key_secret);
    // let verifier = OAuthAddons::Verifier(oauth_verifier);


    
    let bodies = stream::iter(post_ids)
    .map(|id: (String, TweetType)| {
        let client = hyper.clone();
        let token = access_token.clone();
        
        let twitter_url = twitter_url.clone();
        let mut api_path = "tweets";
        let mut request: Option<Request<Body>> = None;
        
        // At the moment, twitter uses OAuth1.0 and 2.0 for Delete Tweets while it only uses 1.0 Authentication for its Unretweets which is a v2 endpoint
        // I really love the OAuth2.0 implementation but need backup code for OAuth1.0 (to handle undo retweets)
        match id.1 {
            TweetType::Tweets => {
                request = Some(RequestBuilder::new(Method::DELETE, format!("{}/2/{}/{}", twitter_url, api_path, id.0))
                    .with_auth(AuthType::Bearer, token).build_request());
            }
            TweetType::Rts => {
                api_path = "statuses/unretweet";
                let base_url = format!("{}/1.1/{}/{}.json", twitter_url, api_path, id.0);
                let signature = OAuth::new(consumer.clone(), Some(oauth_token.clone()), OAuthAddons::None, Method::POST).generate_signature(base_url.clone());
                request = Some(RequestBuilder::new(Method::POST, base_url)
                    .with_auth(AuthType::OAuth, signature.to_string()).build_request());
            }
            TweetType::Likes => {
                todo!()
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
                    // println!("THE BODY {:#?}", body)
                }
                Err(e) => {
                    // includes failed ids in the responsebuilder body?
                    eprintln!("ERROR {:#?}", e)
                }
            }
        }).await;

    return ResponseBuilder::new("Ok".into(), Some(""), 200).reply();
}