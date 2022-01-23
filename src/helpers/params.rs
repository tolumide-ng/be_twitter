use std::{collections::HashMap, borrow::Cow};
use hyper::{Method};

#[derive(Debug, Clone)]
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
    pub fn empty() -> KeyPair {
        KeyPair { key: "".into(), value: "".into() }
    }
}

#[derive(Debug, Clone, Default, derive_more::Deref, derive_more::DerefMut, derive_more::From)]
pub struct RequestParams(HashMap<Cow<'static, str>, Cow<'static, str>>);



impl RequestParams {
    pub fn new() -> Self {
        RequestParams(HashMap::new())
    }

    pub fn add_param(mut self, 
        key: impl Into<Cow<'static, str>>, 
        value: impl Into<Cow<'static, str>>) -> Self 
    {
        self.insert(key.into(), value.into());
        self
    }

    // fn add_param_ref(&mut self, 
    //     key: impl Into<Cow<'static, str>>, 
    //     value: impl Into<Cow<'static, str>>
    // ) {
    //     &self.insert(key.into(), value.into());
    // }

    pub fn add_list_params(
        self, 
        list: Vec<(impl Into<Cow<'static, str>>, impl Into<Cow<'static, str>>)> 
    ) -> Self {
        let mut the_list = self;

        for (key, value) in list {
            the_list = the_list.add_param(key, value);
        };

        the_list
    }

    pub fn add_opt_param(mut self, 
        key: impl Into<Cow<'static, str>>,
        value: Option<impl Into<Cow<'static, str>>>
    ) -> Self {
        match value {
            Some(v) => self.add_param(key.into(), v.into()),
            _ => self
        }
    }
    
}