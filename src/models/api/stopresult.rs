use super::super::stop::Stop;
use super::meta::Meta;

extern crate serde;

use self::serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug,Serialize)]
pub struct StopResult {
    pub result: Vec<Stop>,
    pub meta: Meta
}

#[derive(Debug,Serialize)]
pub struct StopDistanceResult {
    pub result: Vec<StopDistance>,
    pub meta: Meta
}

#[derive(Debug,Serialize)]
pub struct StopDistance {
    pub stop: Stop,
    pub distance: f64
}