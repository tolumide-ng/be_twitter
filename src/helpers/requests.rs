use hyper::Body;
use http::{Method, Request};

use crate::helpers::params::ParamList;

use super::params::KeyPair;

enum OAuthAddOn {
    /// oauth_callback: callback url for generating request token.
    Callback(String),
    /// oauth_verifier: verifier for generating access token.
    Verifier(String),
    None
}


impl OAuthAddOn {
    /// Returns oauth_callback parameter
    fn as_callback(&self) -> Option<&str> {
        match self {
            Self::Callback(url) => Some(url),
            _ => None,
        }
    }

    fn as_verifier(&self) -> Option<&str> {
        match self {
            Self::Verifier(v) => Some(v),
            _ => None
        }
    }
}


pub struct RequestBuilder<'a> {
    base_uri: &'a str,
    method: Method,
    params: Option<ParamList>,
    query: Option<String>,
    body: Option<(Body, &'static str)>,
    addon: OAuthAddOn,
}

impl<'a> RequestBuilder<'a> {
    /// Creates a new `RequestBuilder` with the given HTTP method and base Url
    pub fn new(method: Method, base_uri: &'a str) -> Self {
        Self {
            base_uri,
            method,
            params: None,
            query: None,
            body: None,
            addon: OAuthAddOn::None,
        }
    }

    /// Adds oauth_callback to the OAuth params
    pub fn with_oauth_callback(self, callback: impl Into<String>) -> Self {
        Self {
            addon: OAuthAddOn::Callback(callback.into()),
            ..self
        }
    }

    /// Adds oauth_verifier to the OAuth params
    pub fn with_oauth_verifier(self, verifier: impl Into<String>) -> Self {
        Self {
            addon: OAuthAddOn::Verifier(verifier.into()),
            ..self
        }
    }


    /// Formats the RequestBuilder into a complete `Request`, signing it with the given keys.
    /// 
    /// `token` should only be None when generating a request_token
    pub fn request_keys(self, consumer_key: &KeyPair, token: Option<&KeyPair>) -> Request<Body> {
        // let oauth = Oauth
    }
}