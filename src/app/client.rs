use std::fmt;

use futures::AsyncWriteExt;
use http::{Response};
use hyper::{Client, client::{HttpConnector}, Method, Request, Body, Error, body::HttpBody};
use hyper_tls::HttpsConnector;
use urlencoding::encode;


use crate::helpers::request::RequestBuilder;
use crate::helpers::params::KeyPair;


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
        // &self.get_oauth_request_token(&credentials).await;
        let client = &self.pool.clone();
        // need to write an error converter for this
        let res = client.request(request).await.unwrap();
        let (parts, body) = res.into_parts();
        let body: Vec<_> = hyper::body::to_bytes(body).await.unwrap().to_vec();

        // if let Ok(errors) = serde_json::from_slice() {}
        // Ok((parts.headers, body))
        // Ok(body)

        let body = std::str::from_utf8(&body).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "stream did not contain valid UTF-8",
            )
        }).unwrap();

        println!("THE BODY {}", body);

        println!("OBTAINED PARTS {:#?}", parts);

    }

    pub async fn get_request_token<S: Clone + fmt::Display + Into<String>>(&self, callback: S, consumer: KeyPair) {
        let c = callback.clone().to_owned();


        // let dt = format!("{}", encode(format!("{}", ab).as_str()));

        let request = RequestBuilder::new(Method::POST, "https://api.twitter.com/oauth/request_token")
            .with_oauth_callback(callback.clone().into())
            .with_query(KeyPair::new("oauth_callback", encode(&callback.clone().into()).into_owned()))
            .request_keys(consumer, None);
      
            
        println!("WHAT THE REQUEST LOOKS LIKE {:#?}", request);

        self.make_call(request).await;
    }


    pub async fn get_oauth_authorize(&self) {
        let client = &self.pool.clone();
    }
}
