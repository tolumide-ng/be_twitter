use std::fmt::Debug;
use std::{borrow::Cow, fmt};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{Body, Request, Method};

use crate::middlewares::request_params::RequestParams;
use crate::helpers::keypair::KeyPair;

pub struct RequestBuilder<'a> {
    base_uri: &'a str,
    method: Method,
    params: Option<RequestParams>,
    query: Option<String>,
    body: Option<(Body, &'static str)>,
    header: Option<KeyPair>,
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
            header: None,
            // addon: OAuthAddOn::None,
        }
    }

    pub fn with_query<T: Into<String> + fmt::Display>(self, key: T, value: T) -> Self {
        // if key.len() == 0 || value.len() == 0 {return self}
        let query = match &self.query {
            Some(query) => format!("{}&{}={}", query, key, value),
            None => format!("{}={}", key, value)
        };

        Self {
            query: Some(query),
            ..self
        }
    }

    pub fn get_uri(&self) -> String {
        format!("{}?{}", &self.base_uri.to_string(), &self.query.clone().unwrap_or("".into()))
    }

    
    pub fn with_body(self, body: impl Into<Body>, content: &'static str) -> Self {
        Self {
            body: Some((body.into(), content)),
            ..self
        }
    }

    pub fn with_json_body(self, body: impl serde::Serialize) -> Self {
        self.with_body(
            serde_json::to_string(&body).unwrap(),
            "application/json; charset=UTF-8"
        )
    }


    pub fn request_no_keys(self) -> Request<Body> {
        self.build_request(None)
    }




    // pub fn request_keys(self, consumer: KeyPair, token: Option<KeyPair>) -> Request<Body> {

    //     let oauth = OAuthParams::from_keys(consumer.clone(), token.clone())
    //         .with_addon(self.addon.clone())
    //         .sign_request(&self.method, self.params.as_ref(), &self.base_uri,  self.get_uri());

    //     self.build_reqest(oauth.get_header())
    // }

    fn build_request(self, authorization: Option<String>) -> Request<Body> {
        let url = self.get_uri();

        let request = Request::builder()
            .method(self.method)
            .uri(url);

        let request= match authorization {
            Some(auth) => request.header(AUTHORIZATION, format!("Basic {}", auth)),
            None => request
        };

        if let Some((body, content)) = self.body {
            // println!("THE REQUEST BODY {:#?}", request);
            request.header(CONTENT_TYPE, content).body(body).unwrap()
        } else {
            // println!("THE REQUEST BODY {:#?}", request);
            request.body(Body::from("")).unwrap()
        }
        // .header(AUTHORIZATION, format!("Basic {}", authorization))
        // .body(Body::from("")).unwrap();


        


    }

    pub fn request_authorize(self) {
        
    }
}