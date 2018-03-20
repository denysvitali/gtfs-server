//! Result related structs and implementations

use super::meta::Meta;

#[derive(Debug,Serialize, Deserialize)]
pub struct Result<T> {
    pub result: Option<T>,
    pub meta: Meta
}