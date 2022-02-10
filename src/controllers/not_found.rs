use hyper::{Response, StatusCode, Body};

use crate::helpers::response::{TResult, ApiBody, ResponseBuilder};


pub fn not_found () -> TResult<ApiBody> {
    ResponseBuilder::new("Resorce not found".into(), Some(""), StatusCode::NOT_FOUND.as_u16()).reply()
}
