use dotenv::dotenv;
use std::env;

use crate::helpers::gen_pkce::Pkce;


pub struct SettingsVars {
    pub client_id: String,
    pub response_type: String,
    pub base_url: String,
    pub callback_url: String,
    pub code_challenge: Pkce,
    pub state: String,
    pub app_address: String,
    pub api_key: String,
    pub api_key_secret: String,
    pub client_secret: String,
    pub twitter_v2: String,
    pub twitter_v1: String,
}

impl SettingsVars {

    pub fn new() -> Self {
         dotenv().ok();

        let variables = vec!["STATE_CODE", "RESPONSE_TYPE", "BASE_URL", 
        "CALLBACK_URL", "CLIENT_URL", "CLIENT_SECRET", "APP_ADDRESS", "API_KEY",
        "API_KEY_SECRET", "REQUEST_URL", "TWITTER_V2", "TWITTER_V1"];
        
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
            code_challenge: Pkce::new(),
            state: Self::get_var("STATE_CODE"),
            app_address: Self::get_var("APP_ADDRESS"),
            api_key: Self::get_var("API_KEY"),
            api_key_secret: Self::get_var("API_KEY_SECRET"),
            client_secret: Self::get_var("CLIENT_SECRET"),
            twitter_v2: Self::get_var("TWITTER_V2"),
            twitter_v1: Self::get_var("TWITTER_V1")
        }
    }

    fn get_var(name: &str) -> String {
        env::var(name).unwrap()
    }
}

