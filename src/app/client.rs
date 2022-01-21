use futures::AsyncWriteExt;
use http::{Response};
use hyper::{Client, client::{HttpConnector}, Method, Request, Body, Error, body::HttpBody};
use hyper_tls::HttpsConnector;
use secrecy::{Secret, ExposeSecret};
use url::Url;
use urlencoding::encode;
use uuid::Uuid;
use tokio::io::{self, AsyncWriteExt as _};


use crate::{setup::variables::SettingsVars, helpers::hmac_signature::{Signature}};
use crate::helpers::app_credentials::AuthorizeRequest;

#[derive(Debug)]
pub struct AppClient {
    pool: Client<HttpsConnector<HttpConnector>>,
    signature: Signature,
}


impl AppClient {
    pub fn new(signature: &Signature) -> Self {
        let https = HttpsConnector::new();
        let pool = Client::builder().build::<_, hyper::Body>(https);
        
        Self {
            pool,
            signature: signature.clone(),
        }

    }


    pub async fn make_call(&self, credentials: &AuthorizeRequest) {
        &self.get_oauth_request_token(&credentials).await;
    }

    async fn get_oauth_request_token(&self, credentials: &AuthorizeRequest) {
        let client = &self.pool.clone();
        let SettingsVars { api_key, redirect_uri, .. } = SettingsVars::new();
        let AuthorizeRequest {oauth_nonce, oauth_timestamp, base_url, ..} = credentials;
        
        let signature = self.signature.sig.clone().unwrap().expose_secret().clone();
        
        let req = Request::builder()
            .method(Method::POST)
            .uri(base_url)
            .header("Authorization", format!("OAuth oauth_consumer_key={}", api_key))
            .header("oauth_nonce", oauth_nonce)
            .header("oauth_timestamp", oauth_timestamp.to_string())
            .header("oauth_signature", signature)
            .header("oauth_signature_method", &credentials.oauth_signature_method)
            .header("oauth_version", &credentials.oauth_version)
            .body(Body::from(""))
            .expect("");

        // returns the oauth_token oauth_token_secret and  oauth_callback_confirmed (this must be true)
        let mut res = client.request(req).await.unwrap();

        println!("WE GOT AN ABC {:#?}", res);

        while let Some(next) = res.data().await {
            let chunk = next.unwrap();

            io::stdout().write_all(&chunk).await.unwrap();
            
            println!("DONE WRITING THE CHUNK {:#?}", chunk);
        }

    }


    pub async fn get_oauth_authorize(&self) {
        let client = &self.pool.clone();
    }
}
