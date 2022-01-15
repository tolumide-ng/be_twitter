use url::Url;

use crate::helpers::scope::{Scope, AuthType};
use crate::setup::variables::SettingsVars;

#[derive(Debug)]
pub struct App {
    pub authorize_url: String,
}


impl App {
    pub fn new() -> Self {
        let SettingsVars { client_id, response_type, base_url, redirect_uri, code_challenge, state } = SettingsVars::new();
        let app_scope = Scope::new(AuthType::User);

        let url = Url::parse_with_params(base_url.as_str(), &[
            ("response_type", response_type.as_str()),
            ("client_id", client_id.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
            ("scope", app_scope.as_str()),
            ("state", state.as_str()),
            ("code_challenge", format!("{}", code_challenge).as_str()),
            ("code_challenge_method", "plain"),
        ]).unwrap();


        Self {
            authorize_url: String::from(url.as_str())
        }
    }
}
