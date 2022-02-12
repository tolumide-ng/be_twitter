use hyper::{StatusCode, Body, Request};

use crate::{helpers::response::{TResult, ApiBody, ResponseBuilder}};


pub async fn not_found (_req: Request<Body>) -> TResult<ApiBody> {
    ResponseBuilder::new("Resorce not found".into(), Some(""), StatusCode::NOT_FOUND.as_u16()).reply()
}
