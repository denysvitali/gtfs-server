//! Time related structs and implementations

use super::pickup::PickUp;
use super::dropoff::DropOff;

use importer::NaiveTime;
use importer::NaiveDate;

#[derive(Debug, Serialize)]
pub struct Time {
    pub trip_id : String,
    pub arrival_time: String,
    pub departure_time: String,
    pub stop_id: String,
    pub stop_sequence: i32,
    pub pickup_type: PickUp,
    pub drop_off_type: DropOff,
    pub route_id: String,
    pub service_days: Vec<bool>,
    pub service_uid: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    #[serde(skip_serializing)]
    pub feed_id: String
}