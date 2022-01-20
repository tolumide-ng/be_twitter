use hyper::{Response, StatusCode};

use crate::helpers::response::{ApiResponse, ApiResponseBody};


pub fn not_found () -> ApiResponse {    
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(ApiResponseBody::new(
            "Resorce not found".to_string(), Some("".to_owned())))
}
