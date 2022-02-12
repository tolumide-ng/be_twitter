use std::{collections::HashMap};
use http::Method;
use hyper::{Body, Request};
use redis::{Client as RedisClient};
use futures::{stream, StreamExt};
use serde_json::Value;
use tokio;

use crate::{helpers::{request::HyperClient, response::{TResult, ApiBody, ResponseBuilder, make_request}, signature::{OAuth, OAuthAddons, SignedParams}, keypair::KeyPair}, middlewares::request_builder::RequestBuilder, setup::variables::SettingsVars};

type Ids = HashMap<String, Vec<String>>;

#[derive(Clone, Debug, PartialEq, Copy)]
enum TweetType {
    Tweets,
    Rts,
}

impl std::fmt::Display for TweetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tweets => write!(f, "tweets"),
            Self::Rts => write!(f, "rts"),
        }
    }
}


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

            if total_rts> 50 || total_tweets > 50 {
                panic!("Total tweets or rts cannot be more than 50")
            }

            let mut all_ids: Vec<(String, TweetType)> = vec![];

            for key in expected_keys {
                let ids = s.get(&key.to_string()).unwrap();

                let duplicates = ids.iter()
                    .find(|x| ids.iter().filter(|y| x == y).count() >= 2);

                let empty_string = ids.iter().find(|x| x.len() < 1);

                if duplicates.is_some() || empty_string.is_some() {
                    panic!("{} must be an array of ids (string type) or an empty array", key)
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

struct TweetInfo {}


// rename this module to destory which then contains destory RTs and destory Posts
pub async fn handle_delete(request: Request<Body>, hyper_client: HyperClient, redis_client: RedisClient) -> TResult<ApiBody> {
    let mut con = redis_client.get_async_connection().await?;
    let access_token: String = redis::cmd("GET").arg(&["access_token"]).query_async(&mut con).await?;

    // req body for the ids must be a vector of strings(id of tweets)
    let req_body = request.into_body();
    let  byte_body = hyper::body::to_bytes(req_body).await?.to_owned();
    let body: Ids = serde_json::from_slice(&byte_body)?;

    let post_ids = PostIds::parse(body).0;
    let parallel_requests = post_ids.len();

    let SettingsVars {twitter_v2, api_key, api_key_secret, twitter_v1, ..} = SettingsVars::new();
    // api_key = oauth_consumer_key
    // api_key_secret = oauth_consumer_ecret
    let oauth_verifier = redis::cmd("GET").arg(&["oauth_verifier"]).query_async(&mut con).await.unwrap();
    let oauth_token_key: String = redis::cmd("GET").arg(&["oauth_token"]).query_async(&mut con).await.unwrap();
    let oauth_token_secret = redis::cmd("GET").arg(&["oauth_token_secret"]).query_async(&mut con).await.unwrap();
    let oauth_token = KeyPair::new(oauth_token_key.clone(),oauth_token_secret);
    // let oauth_consumer_key = redis::cmd("GET").arg(&["oauth_consumer_key"]).query_async(&mut con).await.unwrap();

    
    let consumer = KeyPair::new(api_key, api_key_secret);
    let verifier = OAuthAddons::Verifier(oauth_verifier);


    
    let bodies = stream::iter(post_ids)
    .map(|id: (String, TweetType)| {
        let client = hyper_client.clone();
        let token = access_token.clone();
        
        let v2 = twitter_v2.clone();
        let v1 = twitter_v1.clone();
        let mut api_path = "tweets";
        let mut request: Option<Request<Body>> = None;
        
        // At the moment, twitter uses OAuth1.0 and 2.0 for Delete Tweets while it only uses 1.0 Authentication for its Unretweets which is a v2 endpoint
        // I really love the OAuth2.0 implementation but need backup code for OAuth1.0 (to handle undo retweets)
        match id.1 {
            TweetType::Tweets => {
                request = Some(RequestBuilder::new(Method::DELETE, format!("{}/{}/{}", v2, api_path, id.0))
                    .with_access_token("Bearer", token).build_request());
            }
            TweetType::Rts => {
                api_path = "statuses/unretweet";
                let base_uri = format!("{}/1.1/{}/{}.json", v1, api_path, id.0);
                let mut signature = OAuth::new(consumer.clone(), Some(oauth_token.clone()), verifier.clone(), Method::DELETE).generate_signature(base_uri.clone());
                signature.params.push(("oauth_token".into(), oauth_token_key.clone()));

                let mut params = vec![];
                for sig in signature.params {
                    if sig.0 != "oauth_verifier" {
                        params.push(sig)
                    }
                }

                let update_signature = SignedParams { params };

                // let op = SignedParams {params: signatire};
                request = Some(RequestBuilder::new(Method::POST, base_uri)
                    .with_access_token("OAuth", update_signature.to_string()).build_request());
            }
        };

        tokio::spawn(async move {
                println!("THE REQUEST {:#?}", &request);


                let response = make_request(request.unwrap(), client).await.unwrap();
                response.1
            })
        }).buffer_unordered(parallel_requests);

    bodies
        .for_each(|res| async {
            match res {
                Ok(body) => {
                    let body: Value = serde_json::from_slice(&body).unwrap();
                    println!("THE BODY {:#?}", body)
                }
                Err(e) => {
                    eprintln!("ERROR {:#?}", e)
                }
            }
        }).await;




    
    // let req = RequestBuilder::new(Method::POST, format!("https://api.twitter.com/1.1/statuses/destroy/{}.json"))
    return ResponseBuilder::new("Ok".into(), Some(""), 200).reply();
}