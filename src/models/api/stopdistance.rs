use super::super::stop::Stop;

extern crate serde;

use self::serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug,Serialize)]
pub struct StopDistance {
    pub stop: Stop,
    pub distance: f64
}