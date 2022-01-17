use hyper::{Response, StatusCode};

use crate::helpers::response::ApiResponse;

pub fn not_found() -> ApiResponse {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    
    Ok(not_found)
}


// pub fn not_found() -> http::Result<Response<()>> {
//     Response::builder()
//         .status(StatusCode::NOT_FOUND)
//         .body("Resource not found".to_string())
// }