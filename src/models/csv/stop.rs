#[derive(Debug,Deserialize)]
pub struct StopCSV {
    pub stop_id: String,
    pub stop_name: String,
    pub stop_lat: f32,
    pub stop_lon: f32,
    pub location_type: String,
    pub parent_station: String
}