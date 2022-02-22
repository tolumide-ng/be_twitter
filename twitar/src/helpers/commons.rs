use std::borrow::Cow;

use redis::Client as RedisClient;
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Debug, derive_more::Display)]
pub enum GrantType {
    #[display(fmt = "bearer")]
    Bearer,
    #[display(fmt = "authorization_code")]
    Authorization,
    #[display(fmt = "refresh_token")]
    Refresh,
}

#[derive(Debug, PartialEq, Clone, Deserialize, derive_more::Display)]
pub enum AppEnv {
    #[display(fmt = "local")]
    Local,
    #[display(fmt = "test")]
    Test,
    #[display(fmt = "staging")]
    Staging,
    #[display(fmt = "production")]
    Production,
}

impl AppEnv {
    pub fn new(env: String) -> AppEnv {
        let env = env.as_str();

        match env {
            "test" => AppEnv::Test,
            "staging" => AppEnv::Staging,
            "production" => AppEnv::Production,
            _ => AppEnv::Local
        }
    }
}



#[derive(Debug)]
pub enum DBClient {
    Redis(RedisClient),
    Postgres(Pool<Postgres>),
}


impl DBClient {
    fn with_postgres(self) -> Option<Pool<Postgres>> {
        match self {
            DBClient::Postgres(p) => Some(p),
            _ => None
        }
    }

    fn with_redis (self) -> Option<RedisClient> {
        match self {
            DBClient::Redis(r) => Some(r),
            _ => None,
        }
    }
}