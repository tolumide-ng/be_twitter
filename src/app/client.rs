use std::fmt;

use hyper::{Client, client::{HttpConnector}, Method, Request, Body, body::HttpBody};
use hyper_tls::HttpsConnector;
use secrecy::{Secret, ExposeSecret};
use hyper::{Response};


use crate::{middlewares::{
    request_builder::RequestBuilder, oauth_params::OAuthParams
}, setup::variables::SettingsVars};
use crate::helpers::{
    scope::Scope,
    gen_pkce::Pkce,
};


#[derive(Debug)]
pub struct AppClient {
    pool: Client<HttpsConnector<HttpConnector>>,
    // signature: Signature,
}


impl AppClient {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let pool = Client::builder().build::<_, hyper::Body>(https);
        
        Self {
            pool,
            // signature: signature.clone(),
        }

    }


    pub async fn make_call(&self, request: Request<Body>) {
        let client = &self.pool.clone();
        // need to write an error converter for this
        let res = client.request(request).await.unwrap();

        println!("THE RESPONSE OR WHATEVER {:#?}", res);
        let (parts, body) = res.into_parts();
        let body: Vec<_> = hyper::body::to_bytes(body).await.unwrap().to_vec();

        let body = std::str::from_utf8(&body).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "stream did not contain valid UTF-8",
            )
        }).unwrap();

        println!("THIS IS THE BODY {:#?}", body);

    }

    // pub async fn get_request_token<S: Clone + fmt::Display + Into<String>>(&self, callback: S, consumer: KeyPair) {

    //     let request = RequestBuilder::new(Method::POST, "https://api.twitter.com/oauth/request_token")
    //         .with_oauth_callback(callback.clone().into())
    //         .with_query(KeyPair::new("oauth_callback", encode(&callback.clone().into()).into_owned()))
    //         // .with_query(KeyPair::new("oauth_callback", c.into()))
    //         .request_keys(consumer, None);

    //     self.make_call(request).await;
    // }


    pub async fn oauth2_authorize(&self) {

        let SettingsVars {client_id, redirect_uri, state, ..} = SettingsVars::new();

        // let state = &base64::encode(Uuid::new_v4().to_string());
        let pkce = &Pkce::new().to_string();
        // let pkce = Pk

        let request = RequestBuilder::new(Method::GET, "https://twitter.com/i/oauth2/authorize")
            .with_query("response_type", "code")
            .with_query("client_id", client_id.expose_secret())
            .with_query("redirect_uri", &redirect_uri)
            .with_query("scope", &Scope::with_scopes(
                vec![
                // tweet.read%20users.read%20follows.read%20follows.write
                Scope::ReadTweet, Scope::ReadUsers, Scope::ReadFollows, Scope::WriteFollows, 
                // Scope::OfflineAccess,
                // Scope::WriteTweet, Scope::WriteLike
            ]))
            .with_query("state", "state")
            .with_query("code_challenge", "challenge")
            .with_query("code_challenge_method", "plain")
            // .with_body("", "application/x-www-form-urlencoded")
            .request_no_keys();

        println!("THE IS THE ACTUAL REQUEST {:#?}", request);

        self.redirect_user(request).await;
    }

    pub async fn redirect_user(&self, request: Request<Body>) -> Response<Body> {
           Response::builder()
            .status(302)
            .header("Location", "https://twitter.com/i/oauth2/authorize?response_type=code&client_id=wink&redirect_uri=https://127.0.0.1:8080/twitter/oauth&scope=tweet.read%20users.read%20follows.read%20follows.write&state=state&code_challenge=challenge&code_challenge_method=plain")
            .body(Body::empty())
            .unwrap()
    }
}
