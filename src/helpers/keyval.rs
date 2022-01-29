use std::{collections::HashMap, borrow::Cow};
use derive_more;
use http::Uri;

#[derive(Debug, derive_more::Deref, derive_more::DerefMut, derive_more::From, Clone, Default)]
pub struct KeyVal(HashMap<Cow<'static, str>, Cow<'static, str>>);


#[derive(Debug, Clone)]
pub struct AccessToken {
    pub state: String,
    pub  code: String,
}

impl KeyVal {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_keyval(mut self, key: String, val: String) -> Self {
        self.insert(key.into(), val.into());
        self
    }

    pub fn add_list_keyval(mut self, list: Vec<(String, String)>) -> Self {
        for (k, v) in list {
            self.insert(k.into(), v.into());
        }
        self
    }

    // pub fn from_uri_param_to_dict(uri: &Uri) -> Self {}

    pub fn to_urlencode(&self) -> String {
        self.iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>().join("&")
    }

    pub fn to_access_token(&self) -> Result<AccessToken, Box<dyn std::error::Error>> {
        if self.contains_key("state") && self.contains_key("code") {
            let at = AccessToken {
                state: self.get("state").unwrap().to_string(),
                code: self.get("code").unwrap().to_string()
            };

            return Ok(at)
        }

        Err("State or Code in missen in AccessToken")?
    }

    // pub fn to_query_params(&self) -> String {
    //     self.iter().map(|(k, v)| forma)
    // }
}
