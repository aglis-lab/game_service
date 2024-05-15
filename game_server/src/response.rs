use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Response<T> {
    pub error: Option<Error>,
    pub data: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Error {
    pub code: String,
    pub message: String,
}
