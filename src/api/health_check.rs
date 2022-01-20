use hyper::{Response, StatusCode};

use crate::helpers::response::{ApiResponse, ApiResponseBody};


pub fn health_check() -> ApiResponse {
    let ok_body = ApiResponseBody::new("Ok".to_string(), Some("".to_string()));

    Response::builder()
        .status(StatusCode::OK).body(ok_body)
}