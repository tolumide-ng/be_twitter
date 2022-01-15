use dotenv::dotenv;
use std::env;

use crate::helpers::gen_pkce::Pkce;


pub struct SettingsVars {
    client_id: String,
    response_type: String,
    base_url: String,
    redirect_url: String,
    code_challenge: Pkce,
}

impl SettingsVars {

    pub fn new() -> Self {
         dotenv().ok();

        let variables = vec!["STATE_CODE", "RESPONSE_TYPE", "BASE_URL", "REDIRECT_URL"];
        
        // Confirm that all required environment variables are provided
        for variable in variables {
            match env::var(variable) {
                Ok(_) => {}
                Err(_) => panic!("Env Variable: {:?} is required", variable)
            }
        };

        Self {
            client_id: Self::get_var("CLIENT_ID"),
            response_type: Self::get_var("RESPONSE_TYPE"),
            base_url: Self::get_var("BASE_URL"),
            redirect_url: Self::get_var("REDIRECT_URL"),
            code_challenge: Pkce::new(),
        }
    }

    fn get_var(name: &str) -> String {
        env::var(name).unwrap()
    }
}

