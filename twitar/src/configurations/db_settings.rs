use sqlx::postgres::{PgConnectOptions, PgSslMode};

use super::variables::{SettingsVars, AppEnv};

// sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn new(vars: SettingsVars) -> Self {
        let mut require_ssl = true;

        if [AppEnv::Local.to_string(), AppEnv::Test.to_string()].contains(&vars.app_env) {
            require_ssl = false;
        }

        Self {
            username: vars.db_username,
            password: vars.db_password,
            port: vars.db_port,
            host: vars.db_host,
            database_name: vars.db_name,
            require_ssl,
        }
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };


        PgConnectOptions::new().host(&self.host).username(&self.username).password(&self.password).port(self.port).ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}