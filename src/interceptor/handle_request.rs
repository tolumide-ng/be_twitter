use std::collections::HashMap;
use serde_json::Value;

use crate::{
    helpers::response::{TResult, THeaders}, 
    errors::{twitter_errors::TwitterResponseError, response::AppError}
};

// trait TwitterResponse {}

pub struct TwitterInterceptor;

impl TwitterInterceptor {
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
}