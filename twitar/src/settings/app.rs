use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub access_token: String,
    pub access_token_secret: String,
    pub app_address: String,
    pub api_key: String,
    pub api_key_secret: String,
    pub base_url: String,
    pub bearer_token: String,
    pub client_url: String,
    pub client_secret: String,
    pub callback_url: String,
    pub database_url: String,
    pub env: String,
    pub port: String,
    pub request_url: String,
    pub response_type: String,
    pub state_code: String,
    pub twitter_api: String,
}