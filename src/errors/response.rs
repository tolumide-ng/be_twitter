use thiserror;
use redis::RedisError;
use url::ParseError;
use std::{fmt::{self, Formatter}, error};

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


#[derive(thiserror::Error)]
pub enum TError {
    #[error("{0}")]
    ValidationError(String),
    #[error("DataStore error")]
    RedisStoreError(#[from] RedisError),
    #[error("Error parsing query params on uri")]
    BadQueryParams(#[from] ParseError),
    /// This would be called when the state value on the query_params of the redirect_uri does not
    /// match the one originally sent from the application
    #[error("Values do not match")]
    InvalidCredential(&'static str),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl fmt::Debug for TError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       error_chain_fmt(self, f)
    }
}

