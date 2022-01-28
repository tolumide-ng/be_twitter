use std::{collections::HashMap, borrow::Cow};
use derive_more;

#[derive(Debug, derive_more::Deref, derive_more::DerefMut, derive_more::From, Clone, Default)]
pub struct KeyVal(HashMap<Cow<'static, str>, Cow<'static, str>>);


impl KeyVal {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_keyval(self, key: String, val: String) -> Self {
        self.insert(key.into(), val.into());
        self
    }

    pub fn add_list_keyval(self, list: Vec<(String, String)>) -> Self {
        list.into_iter().map(|(k, v)| self.insert(k.into(), v.into()));
        self
    }

    pub fn to_urlencode(&self) -> String {
        self.iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>().join("&")
    }

    // pub fn to_query_params(&self) -> String {
    //     self.iter().map(|(k, v)| forma)
    // }
}