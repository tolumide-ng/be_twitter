use urlencoding::encode;
use uuid::Uuid;
use url::Url;
use hyper::{Client, client::{HttpConnector, ResponseFuture}, Method, Request, Body};

use crate::setup::variables::SettingsVars;

struct AppClient {
    pool: Client<HttpConnector>
}

impl AppClient {
    pub fn new() -> Self {
        let pool = Client::new();
        
        Self {
            pool
        }

    }

    pub fn get_request_token(&self) -> ResponseFuture {
        let client = &self.pool.clone();

        let SettingsVars { client_id, response_type, base_url, redirect_uri, 
            code_challenge, state, client_secret, app_address, api_key, .. } = SettingsVars::new();

        let callback_url = encode(app_address.as_str());
        let oauth_nonce = base64::encode(Uuid::new_v4().to_string());

        let url = Url::parse_with_params(
            "https://api.twitter.com/oauth/request_token", &[
                ("oauth_callback", callback_url),
            ]).unwrap();
        
        let req = Request::builder()
            .method(Method::POST)
            .uri("https://api.twitter.com/oauth/authorize")
            .header("Authorization", format!("OAuth oauth_consumer_key={}", api_key))
            .header("oauth_nonce", oauth_nonce)
            .body(Body::from("Hello"))
            .expect("");

        client.request(req)
    }
}