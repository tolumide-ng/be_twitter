use redis::RedisError;
use std::{fmt, error};


#[derive(Debug)]
pub enum ApiResponseErrors {
    RedisStoreError(RedisError),
}

impl fmt::Display for ApiResponseErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Api request failed")
    }
}

impl error::Error for ApiResponseErrors {}

impl From<RedisError> for ApiResponseErrors {
    fn from(e: RedisError) -> Self {
        Self::RedisStoreError(e)
    }
}