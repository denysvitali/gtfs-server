use super::super::stop::Stop;
use super::meta::Meta;

use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug,Serialize)]
pub struct StopResult {
    pub result: Vec<Stop>,
    pub meta: Meta
}

impl Serialize for StopResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("StopResult", 2)?;
        state.serialize_field("result", &self.result)?;
        state.serialize_field("meta", &self.meta)?;
        state.end()
    }
}