use std::{collections::HashMap, borrow::Cow};
use hyper::{Method};

#[derive(Debug, Clone, Copy)]
pub struct KeyPair {
    pub key: &'static str,
    pub secret: &'static str,
}

impl KeyPair {
    /// Creates KeyPair with the given key and secret.
    pub fn new<K, S>(key: &'static str, secret: &'static str) -> KeyPair {
        KeyPair { key: key.into(), secret: secret.into() }
    }

    // Create an empty KeyPair
    pub fn empty() -> KeyPair {
        KeyPair { key: "".into(), secret: "".into() }
    }
}

