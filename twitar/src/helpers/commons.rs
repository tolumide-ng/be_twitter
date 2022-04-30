use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{errors::response::TError, base_repository::db::{DB, AuthUser, V2User}};

use super::response::TResult;

pub enum GrantType {
    Bearer,
    Authorization,
    Refresh,
}

impl std::fmt::Display for GrantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorization => write!(f, "authorization_code"),
            Self::Bearer => write!(f, "bearer"),
            Self::Refresh => write!(f, "refresh_token"),
        }
    }
}

#[derive(derive_more::Display)]
pub struct UserId(#[display(fmt = "0")] Uuid,);

impl UserId {
    pub fn parse(input: Option<String>) -> TResult<Self> {
        if let Some(id) = input {
            let user_id = Uuid::parse_str(&id)?;
            return Ok(Self(user_id))
        }
        
        Err(TError::InvalidUserId("User id is not present"))
    }

    pub async fn verify(&self, pool: &Pool<Postgres>) -> TResult<AuthUser> {
        let user_exists = DB::user_exists(pool, self.0).await?;

        if let Some(user) = user_exists {
            return Ok(user)
        }

        return Err(TError::InvalidUserId("User does not exist"))
    }

    pub async fn v2_credentials(&self, pool: &Pool<Postgres>) -> TResult<V2User> {
        let user = DB::v2_user(pool, self.0).await?;

        if let Some(credentials) = user {
            return Ok(credentials)
        }
        return Err(TError::InvalidUserId("User dpes not exist"))
    }
}
