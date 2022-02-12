use hyper::{Body, StatusCode, Request};
use routerify::prelude::*;

use crate::{helpers::response::{TResult, ApiBody, ResponseBuilder}, app::server::AppState};


pub async fn health_check(req: Request<Body>) -> TResult<ApiBody> {
    let state = req.data::<AppState>().unwrap();
    ResponseBuilder::new("Ok".into(), Some(""), StatusCode::OK.as_u16()).reply()
}