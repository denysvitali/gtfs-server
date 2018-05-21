//! Stop related structs and implementations
use importer::NaiveDate;
use importer::NaiveTime;

use super::dropoff::DropOff;
use super::pickup::PickUp;
use super::time::Time;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stop {
    pub uid: String,
    #[serde(skip_serializing, skip_deserializing)]
    id: String,
    pub name: String,
    pub lat: f64,
    pub lng: f64,
    pub location_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_station: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    feed_id: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct StopTrip {
    pub stop: Stop,
    pub arrival_time: NaiveTime,
    pub departure_time: NaiveTime,
    pub stop_sequence: i32,
    pub drop_off: DropOff,
    pub pickup: PickUp,
}

impl Stop {
    pub fn new(
        uid: String,
        name: String,
        lat: f64,
        lng: f64,
        location_type: Option<i32>,
        parent_station: Option<String>,
    ) -> Stop {
        let id = String::new();
        let feed_id = String::new();
        Stop {
            uid,
            id,
            name,
            lat,
            lng,
            location_type: match location_type.is_none() {
                true => 0,
                false => location_type.unwrap()
            },
            parent_station,
            feed_id,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_feed_id(&mut self, feed_id: String) {
        self.feed_id = feed_id;
    }
}
