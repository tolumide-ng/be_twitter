// https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature

use urlencoding::encode;
use std::{collections::HashMap, fmt};


#[derive(Default, Debug)]
pub struct Signature {
    parameter_string: Option<String>,
    signature_base_string: Option<String>,
}


// to be removed later -- for practise only
#[derive(Debug, Clone)]
pub struct AuthorizeRequest {
    pub include_entities: String,
    pub oauth_consumer_key: String,
    pub oauth_nonce: String,
    pub oauth_signature_method: String,
    pub oauth_timestamp: String,
    pub oauth_token: String,
    pub oauth_version: String,
    pub base_url: String,
    pub method: ApiCallMethod,
}


#[derive(Debug, Clone)]
pub enum ApiCallMethod {
    POST,
    DELETE,
    GET,
}

impl From<ApiCallMethod> for String {
    fn from(method: ApiCallMethod) -> Self {
        match method {
            ApiCallMethod::DELETE => "DELETE".to_string(),
            ApiCallMethod::GET => "GET".to_string(),
            ApiCallMethod::POST => "POST".to_string(),
        }
    }
}


impl Signature {
    pub fn new(request: AuthorizeRequest) {
        let mut signature: Signature = Default::default();

        signature.parameter_string = Some(signature.get_parameter(request.clone()));
        signature.get_signature_base_string(request.clone());

    }

    fn get_parameter(&self, request: AuthorizeRequest) -> String {
        let mut request_params: HashMap<&str, String> = HashMap::new();
        let AuthorizeRequest {include_entities, oauth_consumer_key, oauth_nonce,
            oauth_signature_method, oauth_timestamp, oauth_token, oauth_version, ..
        } = request;

        request_params.insert("include_entities", include_entities);
        request_params.insert("oauth_consumer_key", oauth_consumer_key);
        request_params.insert("oauth_nonce", oauth_nonce);
        request_params.insert("oauth_signature_method", oauth_signature_method);
        request_params.insert("oauth_timestamp", oauth_timestamp);
        request_params.insert("oauth_token", oauth_token);
        request_params.insert("oauth_version", oauth_version);


        let mut encoded_r_params = vec![];

        for (key, val) in request_params.iter() {
            encoded_r_params.push((encode(key), encode(val)))
        }

        encoded_r_params.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut parameter_string = String::from("");

        encoded_r_params.iter().enumerate().for_each(|(index,val)| {
            let mut key_val = format!("{}={}", val.0, val.1);
            
            if index != encoded_r_params.len() - 1{
                key_val.push_str("&");
            }

            parameter_string.push_str(key_val.as_str());
        });

        return parameter_string
    }

    fn get_signature_base_string(&mut self, request: AuthorizeRequest) {
        let mut base_string = String::from("");

        base_string.push_str(format!("{}&", String::from(request.method)).as_str());
        base_string.push_str(format!("{}&", encode(request.base_url.as_str())).as_str());

        let parameter_string = format!("{}", encode(self.parameter_string.as_ref().unwrap().as_str()));
        base_string.push_str(parameter_string.as_str());

        self.signature_base_string = Some(base_string);

    }
}