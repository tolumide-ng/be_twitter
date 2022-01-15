use url::Url;

use crate::helpers::scopes::Scope;

#[derive(Debug)]
pub struct App {
    authorize_url: String,
}


impl App {
    // pub fn new() -> Self {
    //     let SettingsVars{ base_url, ..} = SettingsVars::new();

    //     Self {
    //         scopes: String::from("tweet.read%20users.read%20follows.read%20follows.write"),
    //         code_challenge: Pkce::new(),
    //         base_url,
    //         // response_type: App::get_state_code(),
    //         response_type: String::from(""),
    //     }
    // }

    fn new() {
        // let app_required_scope = Scope::generate();
    }
}