use std::fmt;

use dotenv::dotenv;
use secrecy::{Secret, ExposeSecret};
use urlencoding;
use uuid::Uuid;

use crate::{setup::variables::SettingsVars, helpers::{gen_pkce::Pkce, scope::Scope}};

pub struct OAuthParams {
    response_type: String,
    client_id: Secret<String>,
    redirect_uri: String,
    state: String,
    code_challenge: String,
    code_challenge_method: String,
    scope: Vec<Scope>,
}


impl OAuthParams {
    pub fn seed() -> Self {
        dotenv().ok();

        let SettingsVars {redirect_uri, client_id, ..} = SettingsVars::new();
        Self {
            response_type: "code".to_string(),
            client_id,
            redirect_uri,
            state: Uuid::new_v4().to_string(),
            code_challenge: Pkce::new().to_string(),
            code_challenge_method: "plain".to_string(),
            scope: vec![],
        }
    }

    pub fn with_permissions(self, scope: Vec<Scope>) -> Self {
        Self {
            scope,
            ..self
        }
    }

    fn encoded_scope(&self) -> String {
        let all_scopes = &self.scope
            .iter().map(|sc| sc.to_string())
            .collect::<Vec<_>>().join(" ");
            
        urlencoding::encode(&all_scopes).to_string()
    }
}


impl fmt::Display for OAuthParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let client_id = &self.client_id.expose_secret().to_string();
        let scopes = &self.encoded_scope();

        let vec_tup = vec![
            ("response_type", &self.response_type), 
            ("client_id", client_id),
            ("redirect_uri", &self.redirect_uri),
            ("scope", scopes),
            ("state", &self.state),
            ("code_challenge", &self.code_challenge),
            ("code_challenege_method", &self.code_challenge_method),
        ];
        let query_str = vec_tup.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>().join("&");
        
        write!(f, "{}", query_str)
    }
}