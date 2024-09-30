use serde::Deserialize;

// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct Requester {
    pub request_id: i32,
    pub name: String,
    pub email: String,
    pub department: String,
}
