use std::{time::SystemTime, collections::HashMap, borrow::Cow};
use http::Method;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use uuid::Uuid;

use crate::{helpers::keypair::KeyPair};

#[derive(Debug, derive_more::Deref, derive_more::DerefMut, derive_more::From, Clone, Default)]
pub struct Params(HashMap<Cow<'static, str>, Cow<'static, str>>);

impl Params {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_param(mut self, key: impl Into<Cow<'static, str>>, val: impl Into<Cow<'static, str>>) -> Self {
        self.insert(key.into(), val.into());
        self
    }

    pub fn add_opt_param(mut self, key: impl Into<Cow<'static, str>>, val: Option<impl Into<Cow<'static, str>>>) -> Self {
        match val {
            Some(v) => {
                self.insert(key.into(), v.into()); 
                self
            }
            None => self
        }
    }
}

#[derive(Debug, Clone)]
pub enum OAuthAddons {
    Callback(String),
    Verifier(String),
    None,
}


impl OAuthAddons {
    pub fn with_callback(&self) -> Option<String> {
        match self {
            Self::Callback(url) => Some(url.to_string()),
            _ => None,
        }
    }

    pub fn with_verifier(&self) -> Option<String> {
        match self {
            Self::Verifier(v) => Some(v.to_string()),
            _ => None,
        }
    }
}


pub struct SignedParams{
    pub params: Vec<(&'static str, String)>
}

impl std::fmt::Display for SignedParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params_str = self.params.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{}", params_str)
    }
}



// impl SignedParams {
//     fn get_header(&self) -> String {
//         let oauth_str = self.params
//             .iter()
//             .map(|(k, v)| format!("{}=\"{}\"", k, urlencoding::encode(v)))
//             .collect::<Vec<String>>()
//             .join(", ");

//         format!("OAuth {}", oauth_str)
//     }
// }

#[derive(Debug)]
pub struct OAuth {
    consumer: KeyPair,
    nonce: String,
    timestamp: String,
    token: Option<KeyPair>,
    addons: OAuthAddons,
    method: String,
}

impl OAuth {
    pub fn new(consumer: KeyPair, token: Option<KeyPair>, addons: OAuthAddons, method: Method,) -> Self {
        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d.as_secs().to_string(),
            Err(e) => panic!("SystemTime before UNIX EPOCH {}", e)
        };

        Self {
            consumer,
            nonce: Uuid::new_v4().to_string(),
            timestamp,
            token,
            addons,
            method: method.to_string().to_uppercase(),
        }
    }

    pub fn generate_signature(self, target_uri: &'static str) -> SignedParams {
        // make hashmap with keys and val

        let token = self.token.clone();

        let params = Params::new()
            .add_opt_param("oauth_callback", self.addons.with_callback().map(|k| k)) // experiment to see if it works if this isn't included
            .add_param("oauth_consumer_key", self.consumer.key.clone())
            .add_param("oauth_nonce", self.nonce.clone())
            .add_param("oauth_signature_method", "HMAC-SHA1")
            .add_param("oauth_timestamp", self.timestamp.clone())
            .add_opt_param("oauth_token", token.clone().map(|k| k.key.clone()))
            .add_param("oauth_version", "1.0");

        let mut encoded_params: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect();

        encoded_params.sort();
        let params_string = encoded_params.join("&");

        // Create signature base_string
        let base_string = format!("{}&{}&{}", 
            self.method, urlencoding::encode(target_uri), 
            urlencoding::encode(&params_string)
        );

        println!("THE BASEIC STRING:::: {}", base_string);

        // Get a signing key
        let secret = match token {
            Some(pair) => {pair.secret}
            None => {String::from("")}
        };

        let key = format!("{}&{}", urlencoding::encode(&self.consumer.secret), urlencoding::encode(&secret));

        // Calculate the signature
        type HmacSha1 = Hmac::<Sha1>;
        let mut mac = HmacSha1::new_from_slice(key.as_bytes()).expect("Wrong key length");
        mac.update(base_string.as_bytes());
        
        let signed_key = base64::encode(mac.finalize().into_bytes());

        let mut all_params = vec![
            ("oauth_consumer_key".into(), self.consumer.key),
            ("oauth_nonce".into(), self.nonce),
            ("oauth_signature".into(), signed_key),
            ("oauth_signature_method".into(), "HMAC-SHA1".to_string()),
            ("oauth_timestamp".into(), self.timestamp),
            ("oauth_version".into(), "1.0".into()),
        ];

        match &self.addons {
            OAuthAddons::Callback(c) => {
                all_params.push(("oauth_callback".into(), c.into()));
            }
            OAuthAddons::Verifier(v) => {
                all_params.push(("oauth_verifier".into(), v.into()));
            }
            OAuthAddons::None => {}
        }

        if let Some(token) = self.token {
            all_params.push(("token".into(), token.secret.clone()));
        }

        SignedParams {
            params: all_params
        }
    }
}