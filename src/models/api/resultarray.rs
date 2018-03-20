//! ResultArray related structs and implementations

use super::meta::Meta;

#[derive(Debug,Serialize,Deserialize)]
pub struct ResultArray<T> {
    pub result: Option<Vec<T>>,
    pub meta: Meta
}