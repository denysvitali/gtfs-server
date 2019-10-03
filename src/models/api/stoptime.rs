//! StopTime related structs and implementations

use super::super::stop::Stop;
extern crate serde;

use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use chrono::NaiveTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopTime {
    pub stop: String,
    pub time: NaiveTime,
    pub trip: String,
}
