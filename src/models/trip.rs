#[derive(Debug, Serialize)]
pub struct Trip {
    pub uid : String,
    pub route_id : String,
    pub service_id : String,
    pub headsign : String,
    #[serde(skip_serializing)]
    trip_id : String,
    pub short_name : String,
    pub direction_id: i32,
    #[serde(skip_serializing)]
    feed_id: String
}

impl Trip {
    pub fn new( uid: String,
            route_id : String,
            service_id: String,
            headsign: String,
            short_name: String,
            direction_id: i32   ) -> Trip
    {
        Trip {
            uid,
            route_id,
            service_id,
            trip_id: String::new(),
            headsign,
            short_name,
            direction_id,
            feed_id: String::new()
        }
    }

    pub fn set_id(&mut self, id : String){
        self.trip_id = id;
    }

    pub fn set_feed_id(&mut self, feed_id: String){
        self.feed_id = feed_id;
    }
}