use std::{time::SystemTime, collections::HashMap, borrow::Cow};
use http::Method;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use uuid::Uuid;

use crate::helpers::keypair::KeyPair;

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


pub struct SignedParams(HashMap<Cow<'static, str>, Cow<'static, str>>);


// impl SignedHeader {
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

        let params = Params::new()
            .add_opt_param("oauth_callback", self.addons.with_callback().map(|k| k)) // experiment to see if it works if this isn't included
            .add_param("oauth_consumer_key", self.consumer.key)
            .add_param("oauth_nonce", self.nonce.clone())
            .add_param("oauth_signature_method", "HMAC-SHA1")
            .add_param("oauth_timestamp", self.timestamp.clone())
            .add_opt_param("oauth_token", self.token.map(|k| k.key))
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
        let secret = match self.token {
            Some(pair) => {pair.secret}
            None => {""}
        };

        let key = format!("{}&{}", urlencoding::encode(&self.consumer.secret), urlencoding::encode(secret));

        // Calculate the signature
        type HmacSha1 = Hmac::<Sha1>;
        let mut mac = HmacSha1::new_from_slice(key.as_bytes()).expect("Wrong key length");
        mac.update(base_string.as_bytes());
        
        let signed_key = base64::encode(mac.finalize().into_bytes());
        let mut all_params = HashMap::new();
        all_params.insert("oauth_consumer_key", self.consumer.key);
        all_params.insert("oauth_nonce", &self.nonce);
        all_params.insert("oauth_signature", &signed_key);
        all_params.insert("oauth_signature_method", "HMAC-SHA1");
        all_params.insert("oauth_timestamp", &self.timestamp);
        all_params.insert("oauth_version", "1.0");

        match &self.addons {
            OAuthAddons::Callback(c) => {
                all_params.insert("oauth_callback", c);
            }
            OAuthAddons::Verifier(v) => {
                all_params.insert("oauth_verifier", v);
            }
            OAuthAddons::None => {}
        }

        if let Some(token) = &self.token {
            all_params.insert("token", token.secret);
        }

        SignedParams(all_params)
    }
}