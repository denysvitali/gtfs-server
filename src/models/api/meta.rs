use super::error::Error;
#[derive(Debug, Serialize)]
pub struct Meta {
    pub success: bool,
    pub error: Error
}