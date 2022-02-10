use http::StatusCode;
use http::status::InvalidStatusCode;
use serde::{Deserialize, Serialize};
use thiserror;
use redis::RedisError;
use url::ParseError;
use std::{fmt::{self, Formatter}, collections::HashMap, borrow::Cow};
use hyper::{Error as HError};
use serde_json;

use crate::helpers::response::THeaders;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;

    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}



#[derive(Debug, Deserialize, Serialize, thiserror::Error)]
pub struct TwitterErrors {
    /// A collection of errors
    pub errors: Vec<TwitterErrorCodes>
}

impl fmt::Display for TwitterErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for e in &self.errors {
            if first {
                first = false;
            } else {
                write!(f, ",")?;
            }

            write!(f, "{}", e)?;
            println!("WHAT THE FN LOOKS LIKE {:#?}", e);
        }


        Ok(())
    }
}


/// Specific errors returned by the Twitter API
#[derive(Debug, Deserialize, Serialize)]
pub struct TwitterErrorCodes {
    /// The error message returned by Twitter.
    pub message: String,
    /// Error code returned by Twitter
    pub code: i32,
}

impl fmt::Display for TwitterErrorCodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TError {
    /// This error is encountered when there is problem deserializing the response body
    #[error("Network Error: {}", 0)]
    NetworkError(#[from] HError),
    #[error("Error parsing query params on uri")]
    BadQueryParamsError(#[from] ParseError),
    #[error("Error processing request: {}", 0)]
    ApiResponseError{message: &'static str},
    #[error("Error Status: {}", _0)]
    BadStatus(hyper::StatusCode),
    #[error("Json Deserialization error: {0}")]
    DeserializeError(#[from] serde_json::Error),
    #[error("Values do not match")]
    InvalidCredentialError(String),
    #[error("Invalid Status code {}", 0)]
    InvalidStatusCode(#[from] InvalidStatusCode),
    #[error("Rate Limit exceeded, please try again in")]
    RateLimit(),
    #[error("DataStore error")]
    RedisStoreError(#[from] RedisError),
    #[error("Error returned by Twitter: {1}")]
    TwitterError(THeaders, TwitterErrors),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("{0}")]
    ValidationError(String),
}



// impl fmt::Debug for TError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         println!("WITHIN THE ERROR CHAIN");
//        error_chain_fmt(self, f)
//     }
// }



#[derive(Debug)]
pub struct AppError(pub HashMap<String, String>, pub u16);


