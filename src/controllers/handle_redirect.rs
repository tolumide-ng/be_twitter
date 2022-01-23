use hyper::{Body, Response, StatusCode};

use crate::helpers::response::{ApiResponse, ApiResponseBody};



pub async fn handle_redirect() -> ApiResponse {
    println!("WE RECEIVED A REQUEST ON THIS ENDPOINT >>>>>>>=========<<<<<<<<<<");

     let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    Response::builder()
        .status(StatusCode::OK).body(ok_body)
}