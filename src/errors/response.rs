use core::fmt;

use redis::RedisError;

#[derive(Debug)]
pub struct RedisStoreError(RedisError);

impl fmt::Display for RedisStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} \n Caused by \n\t{}", self, self.0)
    }
}


pub enum ApiResponseErrors {

}