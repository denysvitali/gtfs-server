//! Trip related structs and implementations
use models::stop::StopTrip;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Clone)]
pub struct Trip {
    pub uid: String,
    pub route_id: String,
    pub service_id: String,
    pub headsign: String,
    #[serde(skip_serializing)]
    trip_id: String,
    pub short_name: String,
    pub direction_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequence: Option<Vec<StopTrip>>,
    #[serde(skip_serializing)]
    feed_id: String,
}

impl Trip {
    pub fn new(
        uid: String,
        route_id: String,
        service_id: String,
        headsign: String,
        short_name: String,
        direction_id: i32,
    ) -> Trip {
        Trip {
            uid,
            route_id,
            service_id,
            trip_id: String::new(),
            headsign,
            short_name,
            direction_id,
            stop_sequence: Some(vec![]),
            feed_id: String::new(),
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.trip_id = id;
    }

    pub fn set_feed_id(&mut self, feed_id: String) {
        self.feed_id = feed_id;
    }
}

impl PartialEq for Trip {
    fn eq(&self, other: &Trip) -> bool {
        self.uid == other.uid
    }
}

// Weakly hashed
// TODO: Implement a better hashing (maybe?)
impl Hash for Trip {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}

impl Eq for Trip {}

impl Ord for Trip {
    fn cmp(&self, other: &Self) -> Ordering {
        self.uid.cmp(&other.uid)
    }
}

impl PartialOrd for Trip {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
