#[derive(Debug, Deserialize)]
pub struct StopTimeCSV {
    pub trip_id: String,
    pub arrival_time: String,
    pub departure_time: String,
    pub stop_id: String,
    pub stop_sequence: i32,
    pub pickup_type: i32,
    pub drop_off_type: i32,
}
