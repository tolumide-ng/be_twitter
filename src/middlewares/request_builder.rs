use std::{fmt};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{Body, Request, Method};

use crate::helpers::keyval::KeyVal;

pub struct RequestBuilder {
    base_uri: String,
    method: Method,
    // params: Option<RequestParams>,
    query: Option<String>,
    body: Option<(Body, &'static str)>,
    header: Option<KeyVal>,
    // addon: OAuthAddOn,
}

impl RequestBuilder {
    pub fn new(method: Method, base_uri: String) -> Self {
        Self {
            base_uri,
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

    pub fn with_header(self, header_dict: Option<KeyVal>) -> Self {
        if let Some(_) = header_dict {
            return Self {
                header: header_dict,
                ..self
            }
        }
        self
    }

    pub fn get_uri(&self) -> String {
        let base_uri = self.base_uri.to_string();
        match &self.query {
            Some(query) => format!("{}?{}", base_uri, query),
            None => base_uri
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

    // combine with_basic_auth, with_access_token, and with_oauth_token - Rename when combined to -with_keys
    pub fn with_basic_auth(self, id: String, secret: String) -> Self {
        let auth_header = base64::encode(format!("{}:{}", id, secret));
        let header_value = format!("Basic {}", &auth_header);
        let header_key = "Authorization".into();
        self.update_header(header_key, header_value)
    }

    pub fn with_access_token(self, prepend: &'static str, access_token: String) -> Self {
        let header_value = format!("{} {}", prepend, access_token);
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