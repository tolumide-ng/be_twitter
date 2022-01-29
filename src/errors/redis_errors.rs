use core::fmt;
use redis::RedisError;

use crate::errors::common::{error_chain_fmt};


pub struct RedisStoreError(RedisError);

impl fmt::Display for RedisStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} \n Caused by \n\t{}", self, self.0)
    }
}


impl std::error::Error for RedisStoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Compiler transparently casts RedisError to dyn::Error
        Some(&self.0)
    }
}


impl std::fmt::Debug for RedisStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        error_chain_fmt(self, f)
    }
}