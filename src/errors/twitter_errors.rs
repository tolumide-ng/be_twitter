use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterResponseError {
    error: String,
    error_description: String,
}