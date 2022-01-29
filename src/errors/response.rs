use core::fmt;

use redis::RedisError;

#[derive(Debug)]
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


pub enum ApiResponseErrors {

}