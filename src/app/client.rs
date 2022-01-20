use http::{Response};
use hyper::{Client, client::{HttpConnector}, Method, Request, Body, Error};
use hyper_tls::HttpsConnector;
use url::Url;
use urlencoding::encode;
use uuid::Uuid;


use crate::setup::variables::SettingsVars;

pub struct AppClient {
    pool: Client<HttpsConnector<HttpConnector>>
}


impl AppClient {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let pool = Client::builder().build::<_, hyper::Body>(https);
        
        Self {
            pool
        }

    }


    pub async fn make_call() {}

    async fn get_oauth_request_token(&self) -> Result<Response<Body>, Error> {
        let client = &self.pool.clone();

        let SettingsVars { app_address, api_key, .. } = SettingsVars::new();

        let oauth_nonce = base64::encode(Uuid::new_v4().to_string());
        let callback_url = encode(app_address.as_str());

        // let _url = Url::parse_with_params(
        //     "https://api.twitter.com/oauth/request_token", &[
        //         ("oauth_callback", callback_url),
        //     ]).unwrap();


        
        
        let req = Request::builder()
            .method(Method::POST)
            .uri("https://api.twitter.com/oauth/authorize")
            .header("Authorization", format!("OAuth oauth_consumer_key={}", api_key))
            .header("oauth_nonce", oauth_nonce)
            .body(Body::from("Hello"))
            // .await()
            .expect("");

        // returns the oauth_token oauth_token_secret and  oauth_callback_confirmed (this must be true)
        client.request(req).await

    }


    pub async fn get_oauth_authorize(&self) {
        let client = &self.pool.clone();
    }
}
