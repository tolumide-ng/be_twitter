use config::{ConfigBuilder, builder::DefaultState, Environment};
use dotenv::dotenv;
use serde::{Deserialize};
use crate::settings::{database::DbSettings, app::AppSettings, variables::AppEnv};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub db: DbSettings,
    pub app: AppSettings,
    pub redis_uri: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    dotenv().ok();
    let app_env: AppEnv = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into().expect("Failed to parse APP_ENV");

     let base_path = std::env::current_dir().expect("Failed to determine the current directory");
     let mut config_dir = base_path.join("configuration");

     if app_env == AppEnv::Test {
        config_dir = base_path.join("../configuration");
     }

     let settings =  ConfigBuilder::<DefaultState>::default()
        .add_source(config::File::from(config_dir.join("base")))
        .add_source(config::File::from(config_dir.join(app_env.to_string())))
        .add_source(Environment::with_prefix("bot").separator("__"))
        .add_source(Environment::with_prefix(&app_env.to_string()).separator("__"));

    settings.build()?.try_deserialize()
}