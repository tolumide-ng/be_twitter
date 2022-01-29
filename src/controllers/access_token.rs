use crate::{
    setup::variables::SettingsVars, 
    helpers::{keyval::KeyVal, request::HyperClient}
};


// todo() - I should move all the controllers used to handle 2.0 authentication into one struct and represent them as methods within the struct
pub async fn access_token(client: HyperClient) {
    let SettingsVars{client_id, redirect_uri, ..} = SettingsVars::new();
    let auth_code= String::from("");

    let req_body = KeyVal::new().add_list_keyval(vec![
        ("grant_type".to_string(), auth_code),
        ("client_id".to_string(), client_id),
        ("redirect_uri".to_string(), redirect_uri),
        ("code_verifier".to_string(), "challenge".to_string())
    ]);

    // let request = RequestBuilder::new(Method::POST, "https://twitter.com/i/oauth2/token");
    
}