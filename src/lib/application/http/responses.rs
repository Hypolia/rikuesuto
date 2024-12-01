use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ErrorResponseData {
    pub message: String,
}
