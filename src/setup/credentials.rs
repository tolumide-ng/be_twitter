use secrecy::{Secret, ExposeSecret};
use url::Url;

use crate::helpers::scope::{Scope, AuthType};
use crate::setup::variables::SettingsVars;

#[derive(Debug)]
pub struct Credentials {
    pub authorize_url: String,
    pub auth_header: String,
}


impl Credentials {
    pub fn new() -> Self {
        let SettingsVars { client_id, response_type, base_url, 
            oauth2_callback, code_challenge, state, client_secret, .. } = SettingsVars::new();
        let app_scope = Scope::new(AuthType::User);

        let url = Url::parse_with_params(base_url.as_str(), &[
            ("response_type", response_type.as_str()),
            ("client_id", client_id.expose_secret().as_str()),
            ("redirect_uri", oauth2_callback.as_str()),
            ("scope", app_scope.as_str()),
            ("state", state.as_str()),
            ("code_challenge", format!("{}", code_challenge).as_str()),
            ("code_challenge_method", "plain"),
        ]).unwrap();


        Self {
            authorize_url: String::from(url.as_str()),
            auth_header: Self::basic_auth(client_id, client_secret)
        }
    }

    fn basic_auth(client_id: Secret<String>, client_secret: Secret<String>) -> String {
        let secret = format!("{}:{}", client_id.expose_secret(), client_secret.expose_secret());
        
        base64::encode(secret)
    }
}
