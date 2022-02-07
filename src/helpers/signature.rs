use std::{time::SystemTime, collections::HashMap, borrow::Cow};
use uuid::Uuid;

use crate::helpers::{keyval::KeyVal};

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

    pub fn add_opt_params(mut self, key: impl Into<Cow<'static, str>>, val: Option<impl Into<Cow<'static, str>>>) -> Self {
        match val {
            Some(v) => {
                self.insert(key.into(), v.into()); 
                self
            }
            None => self
        }
    }
}


#[derive(Debug, Default)]
pub struct OAuth {
    consumer: KeyVal,
    nonce: String,
    timestamp: u64,
    token: Option<KeyVal>,
}

impl OAuth {
    pub fn new(consumer: KeyVal, token: Option<KeyVal>) -> Self {
        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d.as_secs(),
            Err(e) => panic!("SystemTime before UNIX EPOCH {}", e)
        };

        Self {
            consumer,
            nonce: Uuid::new_v4().to_string(),
            timestamp,
            token
        }
    }

    pub fn generate_signature(self, params: Option<&Params>) {
        // make hashmap with keys and val
        let params = Params::new()
            .add_param("include_entities", "true");
    }
}