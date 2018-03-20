//! TimeSearch related structs and implementations
use importer::NaiveDate;
use importer::NaiveTime;
use models::pickup::PickUp;
use models::dropoff::DropOff;


#[derive(FromForm,Serialize,Deserialize)]
pub struct TimeSearch {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub service_uid: Option<String>,
    pub monday: Option<bool>,
    pub tuesday: Option<bool>,
    pub wednesday: Option<bool>,
    pub thursday: Option<bool>,
    pub friday: Option<bool>,
    pub saturday: Option<bool>,
    pub sunday: Option<bool>,
    pub arrival_time_between_a: Option<String>,
    pub arrival_time_between_b: Option<String>,
    pub departure_time_between_a: Option<String>,
    pub departure_time_between_b: Option<String>,
    pub trip_id: Option<String>,
    pub pickup_type: Option<String>,
    pub drop_off_type: Option<String>,
    pub stop_sequence: Option<i32>
}
