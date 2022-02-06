use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitterResponseError {
    pub errors: Vec<HashMap<String, String>>
}