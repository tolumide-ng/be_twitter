use std::{time::SystemTime, collections::HashMap, borrow::Cow};
use urlencoding::encode;
use uuid::Uuid;

use crate::helpers::{keyval::KeyVal};

use super::keypair::KeyPair;

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




#[derive(Debug)]
pub struct OAuth {
    consumer: KeyPair,
    nonce: String,
    timestamp: u64,
    token: Option<KeyPair>,
    addons: OAuthAddons,
}

impl OAuth {
    pub fn new(consumer: KeyPair, token: Option<KeyPair>, addons: OAuthAddons) -> Self {
        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d.as_secs(),
            Err(e) => panic!("SystemTime before UNIX EPOCH {}", e)
        };

        Self {
            consumer,
            nonce: Uuid::new_v4().to_string(),
            timestamp,
            token,
            addons,
        }
    }

    pub fn generate_signature(self, params: Option<&Params>) {
        // make hashmap with keys and val

        let params = Params::new()
            .add_opt_param("oauth_callback", self.addons.with_callback().map(|k| k))
            .add_param("oauth_consumer_key", self.consumer.key)
            .add_param("oauth_nonce", self.nonce)
            .add_param("oauth_signature_method", "HMAC-SHA1")
            .add_param("oauth_timestamp", self.timestamp.to_string())
            .add_opt_param("oauth_token", self.token.map(|k| k.key))
            .add_param("oauth_version", "1.0");

        let mut encoded_params: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect();

        encoded_params.sort();
        let output_str = encoded_params.join("&");

        // create signature base_string

    }
}