//! Meta related structs and implementations

use super::error::Error;
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
