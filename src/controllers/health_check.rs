use hyper::{StatusCode};

use crate::helpers::response::{TResult, ApiBody, ResponseBuilder};


pub async fn health_check() -> TResult<ApiBody> {
    ResponseBuilder::new("Ok".into(), Some(""), StatusCode::OK.as_u16()).reply()
}