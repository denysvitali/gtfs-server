//! TimeSearch related structs and implementations
use importer::NaiveDate;
use importer::NaiveTime;
use models::dropoff::DropOff;
use models::pickup::PickUp;

use super::ascdesc::AscDesc;

#[derive(FromForm, Serialize, Deserialize, Default)]
pub struct TimeSearch {
    pub date: Option<String>,
    pub service_uid: Option<String>,
    pub monday: Option<bool>,
    pub tuesday: Option<bool>,
    pub wednesday: Option<bool>,
    pub thursday: Option<bool>,
    pub friday: Option<bool>,
    pub saturday: Option<bool>,
    pub sunday: Option<bool>,
    pub at_a: Option<String>,
    pub at_b: Option<String>,
    pub dt_a: Option<String>,
    pub dt_b: Option<String>,
    pub trip_id: Option<String>,
    pub pickup_type: Option<String>,
    pub drop_off_type: Option<String>,
    pub stop_sequence: Option<i32>,
    pub sort_by: Option<String>, // TODO: Switch to TimeSort later (https://github.com/SergioBenitez/Rocket/issues/16)
    pub sort_order: Option<String>, // TODO: Switch to AscDesc, see above
    pub stop: Option<String>,
    pub route: Option<String>,
    pub trip: Option<String>,
}

#[derive(Serialize, Deserialize, FromPrimitive, ToPrimitive)]
pub enum TimeSort {
    arrival_time,
    departure_time,
    stop_sequence,
}

impl TimeSort {
    pub fn as_str(&self) -> &str {
        match self {
            &TimeSort::arrival_time => "arrival_time",
            &TimeSort::departure_time => "departure_time",
            &TimeSort::stop_sequence => "stop_sequence",
        }
    }
}
