use hyper::{Response, StatusCode};

use crate::helpers::response::ApiResponseBody;


pub fn not_found() -> http::Result<Response<ApiResponseBody<String>>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(ApiResponseBody {
            message: "Resource not found".to_string()
        })
}