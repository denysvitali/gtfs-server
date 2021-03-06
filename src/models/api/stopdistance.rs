//! StopDistance related structs and implementations

use super::super::stop::Stop;

extern crate serde;

use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct StopDistance {
    pub stop: Stop,
    pub distance: f64,
}
