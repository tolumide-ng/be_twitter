use std::{collections::HashMap, borrow::Cow};
use hyper::{Method};

#[derive(Debug, Clone)]
pub struct KeyPair {
    pub key: Cow<'static, str>,
    pub secret: Cow<'static, str>,
}

impl KeyPair {
    /// Creates KeyPair with the given key and secret.
    pub fn new<K, S>(key: K, secret: S) -> KeyPair 
    where
        K: Into<Cow<'static, str>>,
        S: Into<Cow<'static, str>>,
    {
        KeyPair { key: key.into(), secret: secret.into() }
    }

    // Create an empty KeyPair
    pub fn empty() -> KeyPair {
        KeyPair { key: "".into(), secret: "".into() }
    }
}

