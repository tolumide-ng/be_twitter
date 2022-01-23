use std::{borrow::Cow, fmt};
use hyper::Body;
use http::{Method, Request};

use crate::middlewares::request_params::RequestParams;
use crate::helpers::keypair::KeyPair;

pub struct RequestBuilder<'a> {
    base_uri: &'a str,
    method: Method,
    params: Option<RequestParams>,
    query: Option<String>,
    body: Option<(Body, &'static str)>,
    // addon: OAuthAddOn,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(method: Method, base_uri: &'a str) -> Self {
        Self {
            base_uri,
            method,
            params: None,
            query: None,
            body: None,
            // addon: OAuthAddOn::None,
        }
    }

    // pub fn with_oauth_callback(self, callback: impl Into<String>) -> Self {
    //     Self {
    //         addon: OAuthAddOn::Callback(callback.into()),
    //         ..self
    //     }
    // }

    // pub fn with_oauth_verifier(self, verifier: impl Into<String>) -> Self {
    //     Self {
    //         addon: OAuthAddOn::Verifier(verifier.into()),
    //         ..self
    //     }
    // }

    pub fn with_query(self, query: KeyPair) -> Self {
        Self {
            query: Some(format!("{}?{}={}", self.query.unwrap_or("".into()), query.key, query.value)),
            ..self
        }
    }

    pub fn get_uri(&self) -> String {
        format!("{}{}", &self.base_uri.to_string(), &self.query.clone().unwrap_or("".into()))
    }


    // pub fn request_keys(self, consumer: KeyPair, token: Option<KeyPair>) -> Request<Body> {

    //     let oauth = OAuthParams::from_keys(consumer.clone(), token.clone())
    //         .with_addon(self.addon.clone())
    //         .sign_request(&self.method, self.params.as_ref(), &self.base_uri,  self.get_uri());

    //     self.build_reqest(oauth.get_header())
    // }

    fn build_reqest(self, auth: String) -> Request<Body> {
        let url = self.get_uri();


        let request = Request::builder()
            .method(self.method)
            .uri(url)
            .header("Authorization".to_string(), auth)
            .body(Body::from("")).unwrap();

        println!("THE REQUEST BODY {:#?}", request);
        return request;
    }

    pub fn request_authorize(self) {
        
    }
}