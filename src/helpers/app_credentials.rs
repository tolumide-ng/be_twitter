use secrecy::{Secret};



// to be removed later -- for practise only
#[derive(Debug, Clone)]
pub struct AuthorizeRequest {
    /// Should be boolean but for Rust types // todo!() consider implementing this as an enum for boolean with a From trait
    pub include_entities: String,
    /// Identifies the application making the request (twitter app developer portal)
    // pub oauth_consumer_key: String,
    /// Unique token that should be generated by this app, Twitter will use this value to determine if a request has been submitted multiple times
    pub oauth_nonce: String, 
    /// the oauth signature method used by Twitter is HMAC-SHA1
    pub oauth_signature_method: String,
    /// Number of seconds since Unix epoch at the point the request is generated
    pub oauth_timestamp: u64,
    /// You can initially use the access token of the twitter account that owns the twitter developer app but if you want to make a request on behalf
    /// of a different Twitter account owner, the account owner must grant you access by using the 3-legged OAuth flow (which we are currently 
    /// trying to generate in this case)
    /// More here: https://developer.twitter.com/en/docs/authentication/oauth-1-0a
    pub oauth_token: Option<Secret<String>>,
    pub oauth_version: String,
    pub base_url: String,
    // Must be an http method
    pub method: String,
    // pub oauth_token_secret: Option<Secret<String>>, // OAuth token secret || token secret
}