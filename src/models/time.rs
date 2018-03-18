use super::pickup::PickUp;
use super::dropoff::DropOff;

use importer::NaiveTime;

#[derive(Debug, Serialize)]
pub struct Time {
    pub trip_id : String,
    pub arrival_time: String,
    pub departure_time: String,
    pub stop_id: String,
    pub stop_sequence: i32,
    pub pickup_type: PickUp,
    pub drop_off_type: DropOff,
    #[serde(skip_serializing)]
    feed_id: String
}

impl Time {
    pub fn new( trip_id: String,
                arrival_time : String,
                departure_time: String,
                stop_id: String,
                stop_sequence: i32,
                pickup_type: PickUp,
                drop_off_type: DropOff
    ) -> Time
    {
        Time {
            trip_id,
            arrival_time,
            departure_time,
            stop_id,
            stop_sequence,
            pickup_type,
            drop_off_type,
            feed_id: String::new()
        }
    }

    pub fn set_feed_id(&mut self, feed_id: String){
        self.feed_id = feed_id;
    }
}