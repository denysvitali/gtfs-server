//! StopTime related structs and implementations

use super::super::stop::Stop;
extern crate serde;

use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use chrono::NaiveTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopTimes {
    pub stop: String,
    pub time: Vec<TripTime>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TripTime{
    pub trip: String,
    pub time: NaiveTime
}
