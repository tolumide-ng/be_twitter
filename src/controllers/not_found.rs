use hyper::{Response, StatusCode, Body};

use crate::helpers::response::{ApiResponse, ApiResponseBody};


pub fn not_found () -> ApiResponse {
    let response = ApiResponseBody::new(
"Resorce not found".to_string(), Some("".to_owned()));

    let response_body = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(response)).unwrap();

    Ok(response_body)
}
