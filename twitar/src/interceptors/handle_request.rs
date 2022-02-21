use std::collections::HashMap;
use serde_json::Value;

use crate::{
    helpers::response::{TResult, THeaders}, 
    errors::{twitter_errors::TwitterResponseError, response::AppError}
};


#[derive(Debug)]
pub struct V2Tokens {
    access_token: String,
    refresh_token: String,
}

pub enum V2TokensType {
    Access,
    Refresh
}

impl V2Tokens {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self { access_token, refresh_token }
    }

    pub fn get(&self, token: V2TokensType) -> String {
        match token {
            V2TokensType::Access => self.access_token.clone(),
            V2TokensType::Refresh => self.refresh_token.clone(),
        }
    }
}

pub struct Interceptor;

impl Interceptor {
    pub fn intercept(res: TResult<(THeaders, Vec<u8>)>) -> Result<Value, AppError> { 
        let mut obj = HashMap::new();

        match res {
            Ok(resp) => {
                let (_header, body) = resp;
                let response: Value = serde_json::from_slice(&body).unwrap();

                if response["errors"] != Value::Null {
                    let err: TwitterResponseError = serde_json::from_slice(&body).unwrap();
                    let detail = err.errors[0].get("detail").unwrap();
                    obj.insert("detail".into(), detail.to_string());
                    return Err(AppError(obj, 400));
                }

                let data: Value = serde_json::from_slice(&body).unwrap();
                return Ok(data);

            }
            Err(e) => {
                println!("THE ERROR OBTAINED!!!!!!!!!!!!!!!!!!! {:#?}", e);
                obj.insert("detail".into(), "Internal Server Error".into());
                return Err(AppError(obj, 500));
            }
        }
    }

    pub fn v2_tokens(map: Result<Value, AppError>) -> Option<V2Tokens> {
        if let Ok(body) = map {
            let map: HashMap<String, Value> = serde_json::from_value(body).unwrap();
            
            let has_access_token = map.get("access_token");
            let has_refresh_token = map.get("refresh_token");

            if has_access_token.is_some() && has_refresh_token.is_some() {
                let a_t = map.get("access_token").unwrap().clone();
                let r_t = map.get("refresh_token").unwrap().clone();
                let access_token: String = serde_json::from_value(a_t).unwrap();
                let refresh_token: String = serde_json::from_value(r_t).unwrap();
                
                let tokens = V2Tokens::new(access_token, refresh_token);
                return Some(tokens)
            }
        }
        
        return None
    }
}
