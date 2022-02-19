use dotenv::dotenv;
use serde::Deserialize;
use std::env;

use crate::helpers::gen_pkce::Pkce;

#[derive(Debug, Clone, Deserialize, derive_more::Display)]
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


#[derive(Debug, Clone, Deserialize)]
pub struct SettingsVars {
    pub client_id: String,
    pub response_type: String,
    pub base_url: String,
    pub callback_url: String,
    pub code_challenge: String,
    pub state: String,
    pub app_address: String,
    pub api_key: String,
    pub api_key_secret: String,
    pub client_secret: String,
    pub twitter_url: String,
    pub app_env: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
}

impl SettingsVars {

    pub fn new() -> Self {
         dotenv().ok();

        let variables = vec!["STATE_CODE", "RESPONSE_TYPE", "BASE_URL", 
        "CALLBACK_URL", "CLIENT_URL", "CLIENT_SECRET", "APP_ADDRESS", "API_KEY",
        "API_KEY_SECRET", "REQUEST_URL", "TWITTER_API", "APP_ENV", "DB_HOST", 
        "DB_PORT", "DB_USERNAME", "DB_PASSWORD", "DB_NAME"];
        
        // Confirm that all required environment variables are provided
        for variable in variables {
            match env::var(variable) {
                Ok(_) => {}
                Err(_) => panic!("Env Variable: {:?} is required", variable)
            }
        };

        Self {
            client_id: Self::get_var("CLIENT_URL"),
            response_type: Self::get_var("RESPONSE_TYPE"),
            base_url: Self::get_var("BASE_URL"),
            callback_url: Self::get_var("CALLBACK_URL"),
            code_challenge: Pkce::new().to_string(),
            state: Self::get_var("STATE_CODE"),
            app_address: Self::get_var("APP_ADDRESS"),
            api_key: Self::get_var("API_KEY"),
            api_key_secret: Self::get_var("API_KEY_SECRET"),
            client_secret: Self::get_var("CLIENT_SECRET"),
            twitter_url: Self::get_var("TWITTER_API"),
            app_env: Self::get_var("APP_ENV"),
            db_host: Self::get_var("DB_HOST"),
            db_port: Self::get_var("DB_PORT").parse::<u16>().unwrap(),
            db_username: Self::get_var("DB_USERNAME"),
            db_password: Self::get_var("DB_PASSWORD"),
            db_name: Self::get_var("DB_NAME"),
        }
    }

    fn get_var(name: &str) -> String {
        env::var(name).unwrap()
    }
}

