use hyper::{Body, Response, StatusCode, Request};
use http::Uri;
use url::Url;
use crate::{helpers::{response::{ApiResponse, ApiResponseBody}, request::get_param_from_uri}, setup::variables::SettingsVars};



pub async fn handle_redirect(req: Request<hyper::Body>) -> ApiResponse {
    println!("WE RECEIVED A REQUEST ON THIS ENDPOINT >>>>>>>=========<<<<<<<<<< {:#?}", req);

    // let SettingsVars{state, ..} = SettingsVars::new();
    let auth_code = get_param_from_uri(req.uri(), "code");
    if auth_code.is_some() {
        println!("THE AUTHCODE {:#?}", auth_code)
    }

     let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    Response::builder()
        .status(StatusCode::OK).body(ok_body)
}