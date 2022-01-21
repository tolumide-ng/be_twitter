use std::{collections::HashMap, borrow::Cow};

pub struct KeyPair {
    /// A key used to identify an application or user.
    pub key: Cow<'static, str>,
    /// A private key used to sign messages from an application or user.
    pub value: Cow<'static, str>,
}

impl KeyPair {
    /// Creates KeyPair with the given key and secret.
    pub fn new<K, S>(key: K, secret: S) -> KeyPair 
    where
        K: Into<Cow<'static, str>>,
        S: Into<Cow<'static, str>>,
    {
        KeyPair { key: key.into(), value: secret.into() }
    }

    // Create an empty KeyPair
    fn empty() -> KeyPair {
        KeyPair { key: "".into(), value: "".into() }
    }
}

pub struct ParamList(HashMap<Cow<'static, str>, Cow<'static, str>>);