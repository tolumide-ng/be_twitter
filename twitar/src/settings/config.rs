use serde::{Deserialize};
use crate::settings::{database::DbSettings, app::AppSettings, variables::AppEnv};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub app: AppSettings,
    pub redis_uri: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    dotenv().ok();
    let app_env: AppEnv = std::env::var("APP_ENV");
}