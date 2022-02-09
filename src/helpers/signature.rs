use std::{time::SystemTime, collections::HashMap, borrow::Cow};
use http::Method;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use uuid::Uuid;

use crate::{helpers::keypair::KeyPair};
use crate::helpers::utils::percent_encode;

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
    pub params: Vec<(String, String)>
}

impl std::fmt::Display for SignedParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params_str = self.params.iter()
            .map(|(k, v)| format!(r#"{}="{}""#, percent_encode(k), percent_encode(v)))
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

        let nonce = Uuid::new_v4().to_string().replace("-", ""); 
        

        Self {
            consumer,
            nonce,
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
            .add_param("oauth_consumer_key", self.consumer.key.clone())
            .add_param("oauth_nonce", self.nonce.clone())
            .add_param("oauth_signature_method", "HMAC-SHA1")
            .add_param("oauth_timestamp", self.timestamp.clone())
            .add_param("oauth_version", "1.0")
            .add_opt_param("oauth_token", token.clone().map(|k| k.key.clone()))
            .add_opt_param("oauth_callback", self.addons.with_callback().map(|k| k))
            .add_opt_param("oauth_verifier", self.addons.with_verifier().map(|k| k)); // experiment to see if it works if this isn't included

        let mut query: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
            .collect();
        query.sort();

        println!("::::::::::::::::::::::::: {:#?} :::::::::::::::::::::::::::", query);
        let params_string = query.join("&");

        // Create signature base_string
        let base_string = format!(
            "{}&{}&{}", 
            percent_encode(&self.method), 
            percent_encode(target_uri), 
            percent_encode(&params_string)
        );

        println!("\nTHE BASEIC STRING:::: {}", base_string);

        // Get a signing key
        let secret = match token {
            Some(pair) => {pair.secret}
            None => {String::from("")}
        };

        let key = format!("{}&{}", percent_encode(&self.consumer.secret), percent_encode(&secret));

        // Calculate the signature
        type HmacSha1 = Hmac::<Sha1>;
        let mut mac = HmacSha1::new_from_slice(key.as_bytes()).expect("Wrong key length");
        mac.update(base_string.as_bytes());
        
        let signed_key = base64::encode(mac.finalize().into_bytes());

        let mut all_params = vec![
            ("oauth_consumer_key".to_string(), self.consumer.key),
            ("oauth_nonce".into(), self.nonce),
            ("oauth_signature".into(), signed_key),
            ("oauth_signature_method".into(), "HMAC-SHA1".to_string()),
            ("oauth_timestamp".into(), self.timestamp),
            ("oauth_version".into(), "1.0".into()),
        ];

        match &self.addons {
            OAuthAddons::Callback(c) => {
                all_params.push(("oauth_callback".into(), percent_encode(c).to_string()));
            }
            OAuthAddons::Verifier(v) => {
                all_params.push(("oauth_verifier".into(), v.into()));
            }
            OAuthAddons::None => {}
        }

        if let Some(token) = self.token {
            all_params.push(("token".into(), token.secret.clone()));
        }

        all_params.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        SignedParams {
            params: all_params
        }
    }
}