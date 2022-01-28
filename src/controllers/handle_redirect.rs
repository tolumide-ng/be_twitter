use hyper::{Body, Response, StatusCode, Request};
use crate::{helpers::{response::{ApiResponse, ApiResponseBody}, request::get_param_from_uri}, setup::variables::SettingsVars};



// todo() - I should move all the controllers used to handle 2.0 authentication into one struct and represent them as methods within the struct
pub async fn handle_redirect(req: Request<hyper::Body>) -> ApiResponse {
    let SettingsVars{state, ..} = SettingsVars::new();

    let query_params = get_param_from_uri(req.uri());
    if let Some(dict) = query_params {
        //todo() find a way to confirm if code and state are present in the dic, handle the edge cases
        let obtained_state = dict.get("state").unwrap();
        if *obtained_state != state {
            panic!("Please try again later, state isn't same, CSRF?")
        }
        let auth_code = dict.get("code").unwrap();
    };

     let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    Response::builder()
        .status(StatusCode::OK).body(ok_body)
}