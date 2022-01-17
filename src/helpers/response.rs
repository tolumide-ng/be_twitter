use hyper::{Response, Body};

pub type ApiResponse = Result<Response<Body>, hyper::Error>;

pub struct ApiResponseBody<T: Clone> {
    message: T
}