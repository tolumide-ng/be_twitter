use thiserror;
use redis::RedisError;
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
pub enum ApiResponseErrors {
    #[error("{0}")]
    ValidationError(String),
    #[error("DataStore error")]
    RedisStoreError(#[from] RedisError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl fmt::Debug for ApiResponseErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       error_chain_fmt(self, f)
    }
}

