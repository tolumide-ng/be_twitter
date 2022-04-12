use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{errors::response::TError, base_repository::oauth_2::DbAuth2};

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
    pub fn parse(input: Option<&str>) -> TResult<Uuid> {
        if let Some(id) = input {
            let user_id = Uuid::parse_str(id)?;
            return Ok(user_id)
        }
        
        Err(TError::InvalidUserId("User id is not present"))
    }

    pub async fn verify(user_id: Uuid, pool: &Pool<Postgres>) -> TResult<Self> {
        let user_exists = DbAuth2::user_exists(pool, user_id).await?;

        if user_exists {
            return Ok(Self(user_id));
        }
        
        return Err(TError::InvalidUserId("User does not exist"))
    }
}
