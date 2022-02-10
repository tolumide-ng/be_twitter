use std::{collections::HashMap, borrow::Cow};
use derive_more;
use hyper::{Uri};
use url::{Url};

use crate::{helpers::response::TResult, errors::response::TError};
use crate::controllers::handle_redirect::AccessToken;

#[derive(Debug, derive_more::Deref, derive_more::DerefMut, derive_more::From, Clone, Default)]
pub struct KeyVal(HashMap<Cow<'static, str>, Cow<'static, str>>);



impl KeyVal {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_keyval(mut self, key: String, val: String) -> Self {
        self.insert(key.into(), val.into());
        self
    }

    pub fn new_with_keyval(key: String, val: String) -> Self {
        let mut dict = Self::new();
        dict.insert(key.into(), val.into());

        dict
    }

    pub fn add_list_keyval(mut self, list: Vec<(String, String)>) -> Self {
        for (k, v) in list {
            self.insert(k.into(), v.into());
        }
        self
    }

    pub fn query_params_to_keyval(uri: &Uri) -> TResult<Self> {
        let mut uri_string = uri.to_string();

        if !uri_string.starts_with("https:/") {
            uri_string = format!("https:/{}", uri_string);
        }

        let parsed_uri = Url::parse(&uri_string)?;

        let mut dic = Self::new();

        if let Some(all_qs) = parsed_uri.query() {
            let params: Vec<&str> = all_qs.split("&").collect();

            for param in params {
                let vec_param = param.split("=").collect::<Vec<_>>();
                dic = dic.add_keyval(vec_param[0].into(), vec_param[1].into());
            }
        }

        Ok(dic)
    }

    pub fn to_urlencode(&self) -> String {
        self.iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>().join("&")
    }

    pub fn to_access_token(&self) -> TResult<AccessToken> {
        if self.contains_key("state") && self.contains_key("code") {
            let at = AccessToken {
                state: self.get("state").unwrap().to_string(),
                code: self.get("code").unwrap().to_string()
            };

            return Ok(at)
        }

        Err(TError::InvalidCredentialError("State or Code in missen in AccessToken".into()))
    }
    
    // pub fn to_query_params(&self) -> String {
    //     self.iter().map(|(k, v)| forma)
    // }

    pub fn validate(&self, name: String, value: String) -> bool {
        if let Some(obtained_value) = &self.get(name.as_str()) {
            if obtained_value.to_string() == value {
                // return Ok("akld".into())
                return true;
            }

            return false;
        }

        return false;
    }

    pub fn verify_present(&self, names: Vec<String>) -> Option<&Self> {
        let keys = self.keys().cloned().map(|k| k.to_string()).collect::<Vec<String>>();
        let mut err = false;

        for index in 0..names.len() {
            if !keys.contains(&names[index]) {
                err = true;

            }
        }

        if err {
            return None
        }
        
        Some(&self)
    }

    // pub fn get_from(self, name: String) -> String {
    //     return self.get(name.as_str()).unwrap()
    // }

    // pub fn validate_multiple(&self, values: Vec<String>) {
    //     let mut errors: Vec<String> = vec![];

    //     for value in values {
    //         // if let Some(current_value)
    //     }
    // }
}
