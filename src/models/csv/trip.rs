#[derive(Debug, Deserialize)]
pub struct TripCSV {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: String,
    pub trip_short_name: String,
    pub direction_id: Option<i32>,
    pub block_id: Option<i32>,
    pub shape_id: Option<i32>,
    pub wheelchair_accessible : Option<i32>,
    pub bikes_allowed: Option<i32>
}
