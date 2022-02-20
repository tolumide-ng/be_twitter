use std::{fmt};
use http::header::{CONTENT_TYPE};
use hyper::{Body, Request, Method};

use crate::helpers::keyval::KeyVal;

pub struct RequestBuilder {
    base_url: String,
    method: Method,
    query: Option<String>,
    body: Option<(Body, &'static str)>,
    header: Option<KeyVal>,
}

pub enum ContentType {}


#[derive(Debug, Clone, derive_more::Display, PartialEq)]
pub enum AuthType {
    #[display(fmt = "Bearer")]
    Bearer,
    #[display(fmt = "OAuth")]
    OAuth,
    #[display(fmt = "Basic")]
    Basic,
}

impl RequestBuilder {
    pub fn new(method: Method, base_url: String) -> Self {
        Self {
            base_url,
            method,
            query: None,
            body: None,
            header: None,
            // addon: OAuthAddOn::None,
        }
    }

    pub fn with_query<T: Into<String> + fmt::Display>(self, key: T, value: T) -> Self {
        let query = match &self.query {
            Some(query) => format!("{}&{}={}", query, key, value),
            None => format!("{}={}", key, value)
        };

        Self {
            query: Some(query),
            ..self
        }
    }

    pub fn add_query_params(self, query_dict: KeyVal) -> Self {
        let query_str = query_dict
            .iter().map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>().join("&");

        Self {
            query: Some(query_str),
            ..self
        }
    }

    pub fn with_header(self, header_dict: KeyVal) -> Self {
        return Self {
            header: Some(header_dict),
            ..self
        }
    }

    pub fn get_uri(&self) -> String {
        let base_url = self.base_url.to_string();
        match &self.query {
            Some(query) => format!("{}?{}", base_url, query),
            None => base_url
        }
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

    fn update_header(self, key: String, val: String) -> Self {
        let updated_header = match self.header {
            Some(header) => header.add_keyval(key, val),
            None => KeyVal::new_with_keyval(key, val)
        };

        Self {
            header: Some(updated_header),
            ..self
        }
    }

    pub fn with_auth(self, prepend: AuthType, credentials: String) -> Self {
        let mut value = credentials;
        
        if prepend == AuthType::Basic {
            value = base64::encode(value);
        }

        let header_value = format!("{} {}", prepend, value);
        let header_key = "Authorization".into();
        self.update_header(header_key, header_value)
    }

    // pub fn with_oauth_token(self, oauth_token: )


    pub fn build_request(self) -> Request<Body> {
        let uri = self.get_uri();
        let mut request = Request::builder()
            .method(self.method)
            .uri(uri);

        if let Some(header_map) = self.header {
            for (k, v) in header_map.iter() {
                request = request.header(k.to_string(), v.to_string());
            }
        }

        if let Some((body, content)) = self.body {
            request.header(CONTENT_TYPE, content).body(body).unwrap()
        } else {
            request.body(Body::from("")).unwrap()
        }
    }
}