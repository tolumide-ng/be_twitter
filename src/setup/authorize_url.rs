use url::Url;

use crate::helpers::scopes::{Scope, AuthType};

use super::variables::SettingsVars;

#[derive(Debug)]
pub struct App {
    authorize_url: String,
}


impl App {
    pub fn new() -> Url {
        // let app_required_scope = Scope::generate();
        let SettingsVars { client_id, response_type, base_url, redirect_uri, code_challenge } = SettingsVars::new();
        
        let app_scope = Scope::new(AuthType::User);

        let mut authorize_url = Url::parse(&base_url).unwrap();

        let response_type = format!("response_type={}", response_type);
        let client_id = format!("client_id={}", client_id);
        let redirect_uri = format!("redirect_uri={}", redirect_uri);
        let scope = format!("scope={}", app_scope);
        let code_challenge = format!("code_challenge={}", code_challenge);
        let code_challenge_method = format!("code_challenge_method {}", "plain");
        
        authorize_url.set_query(Some(&response_type));
        authorize_url.set_query(Some(&client_id));
        authorize_url.set_query(Some(&redirect_uri));
        authorize_url.set_query(Some(&scope));
        authorize_url.set_query(Some(&code_challenge));
        authorize_url.set_query(Some(&code_challenge_method));

        
        authorize_url

        // authorize_url.unwrap().set_query("response_type", variables.response_type);
    }
}