use hyper::{Response, Body};

pub type ApiResponse = Result<Response<Body>, hyper::Error>;