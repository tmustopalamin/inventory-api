use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseDataSuccess<T> {
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataError {
    pub message: String,
    pub code: String,
}