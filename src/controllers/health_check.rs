use hyper::{Response, StatusCode, Body};

use crate::helpers::response::{TResult, ApiResponseBody, ApiBody};


pub fn health_check() -> TResult<ApiBody> {
    let ok_body = Body::from(ApiResponseBody::new("Ok".to_string(), Some("".to_string())));

    let response_body = Response::builder()
        .status(StatusCode::OK).body(ok_body).unwrap();

    Ok(response_body)
}